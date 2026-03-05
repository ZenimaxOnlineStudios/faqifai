use std::path::{Path, PathBuf};

use anyhow::Result;
use starlark::any::ProvidesStaticType;
use starlark::environment::{GlobalsBuilder, Module};
use starlark::eval::Evaluator;
use starlark::starlark_module;
use starlark::syntax::{AstModule, Dialect};
use starlark::values::dict::AllocDict;
use starlark::values::list::AllocList;
use starlark::values::{Heap, Value};

use crate::codebase;

/// Context passed to Starlark native functions via `eval.extra`.
/// Holds the workspace root for sandboxed file access.
#[derive(ProvidesStaticType)]
struct EvalContext {
    root: PathBuf,
}

/// Evaluate a Starlark script with sandboxed codebase access functions.
/// Returns the string representation of the script's result.
pub fn evaluate(root: &Path, script: &str) -> Result<String> {
    let ctx = EvalContext {
        root: root.to_path_buf(),
    };

    let globals = GlobalsBuilder::standard()
        .with(codebase_builtins)
        .build();

    let module = Module::new();
    let mut eval = Evaluator::new(&module);
    eval.extra = Some(&ctx);

    let dialect = Dialect {
        enable_top_level_stmt: true,
        enable_f_strings: true,
        ..Dialect::Standard
    };
    let ast = AstModule::parse("eval", script.to_owned(), &dialect)
        .map_err(|e| anyhow::anyhow!("Parse error: {}", e))?;
    let result = eval
        .eval_module(ast, &globals)
        .map_err(|e| anyhow::anyhow!("Eval error: {}", e))?;

    match result.unpack_str() {
        Some(s) => Ok(s.to_string()),
        None => Ok(result.to_repr()),
    }
}

fn get_ctx<'a, 'e>(eval: &Evaluator<'_, 'a, 'e>) -> &'a EvalContext {
    eval.extra
        .expect("EvalContext not set")
        .downcast_ref::<EvalContext>()
        .expect("extra is not EvalContext")
}

#[starlark_module]
fn codebase_builtins(builder: &mut GlobalsBuilder) {
    /// Read the contents of a file. Path is relative to workspace root.
    fn read_file<'v>(
        #[starlark(require = pos)] path: &str,
        eval: &mut Evaluator<'v, '_, '_>,
    ) -> anyhow::Result<String> {
        let ctx = get_ctx(eval);
        let (content, _hash) = codebase::read_file(&ctx.root, path)?;
        Ok(content)
    }

    /// Read specific lines from a file (1-indexed, inclusive).
    fn read_file_lines<'v>(
        #[starlark(require = pos)] path: &str,
        #[starlark(require = pos)] start: i32,
        #[starlark(require = pos)] end: i32,
        eval: &mut Evaluator<'v, '_, '_>,
    ) -> anyhow::Result<String> {
        let ctx = get_ctx(eval);
        let (content, _hash) =
            codebase::read_file_lines(&ctx.root, path, start as usize, end as usize)?;
        Ok(content)
    }

    /// Search for a regex pattern across repository files.
    /// Returns formatted results with file paths and line numbers.
    fn grep<'v>(
        #[starlark(require = pos)] pattern: &str,
        #[starlark(default = "")] glob: &str,
        #[starlark(default = 50)] max_results: i32,
        eval: &mut Evaluator<'v, '_, '_>,
    ) -> anyhow::Result<String> {
        let ctx = get_ctx(eval);
        let glob_filter = if glob.is_empty() { None } else { Some(glob) };
        codebase::grep(&ctx.root, pattern, glob_filter, max_results as usize, 0)
    }

    /// Find files matching a glob pattern. Returns a list of relative paths.
    fn find_files<'v>(
        #[starlark(require = pos)] pattern: &str,
        eval: &mut Evaluator<'v, '_, '_>,
    ) -> anyhow::Result<Vec<String>> {
        let ctx = get_ctx(eval);
        let output = codebase::find_files(&ctx.root, pattern)?;
        let paths: Vec<String> = output
            .lines()
            .filter(|l| !l.is_empty() && !l.ends_with("matched."))
            .map(|l| l.to_string())
            .collect();
        Ok(paths)
    }

    /// List directory contents. Returns a list of entry names (dirs have trailing /).
    fn list_dir<'v>(
        #[starlark(require = pos)] path: &str,
        eval: &mut Evaluator<'v, '_, '_>,
    ) -> anyhow::Result<String> {
        let ctx = get_ctx(eval);
        codebase::list_directory(&ctx.root, path)
    }

    /// Split a string into a list of lines.
    fn lines(#[starlark(require = pos)] text: &str) -> anyhow::Result<Vec<String>> {
        Ok(text.lines().map(|l| l.to_string()).collect())
    }

    /// Parse a JSON string into a Starlark value (dict/list/string/int/bool/None).
    fn json_parse<'v>(
        #[starlark(require = pos)] text: &str,
        eval: &mut Evaluator<'v, '_, '_>,
    ) -> anyhow::Result<Value<'v>> {
        let json_val: serde_json::Value = serde_json::from_str(text)?;
        Ok(json_to_starlark(eval.heap(), &json_val))
    }

    /// Parse a TOML string into a Starlark value (dict/list/string/int/bool).
    fn toml_parse<'v>(
        #[starlark(require = pos)] text: &str,
        eval: &mut Evaluator<'v, '_, '_>,
    ) -> anyhow::Result<Value<'v>> {
        let toml_val: toml::Value = toml::from_str(text)?;
        Ok(toml_to_starlark(eval.heap(), &toml_val))
    }

    /// Find all regex matches in text. Returns list of matched strings.
    fn regex_find(
        #[starlark(require = pos)] pattern: &str,
        #[starlark(require = pos)] text: &str,
    ) -> anyhow::Result<Vec<String>> {
        let re = regex::Regex::new(pattern)
            .map_err(|e| anyhow::anyhow!("Invalid regex '{}': {}", pattern, e))?;
        Ok(re
            .find_iter(text)
            .map(|m| m.as_str().to_string())
            .collect())
    }

    /// Test if a regex pattern matches anywhere in the text.
    fn regex_match(
        #[starlark(require = pos)] pattern: &str,
        #[starlark(require = pos)] text: &str,
    ) -> anyhow::Result<bool> {
        let re = regex::Regex::new(pattern)
            .map_err(|e| anyhow::anyhow!("Invalid regex '{}': {}", pattern, e))?;
        Ok(re.is_match(text))
    }
}

/// Convert a serde_json::Value to a Starlark Value
fn json_to_starlark<'v>(heap: &'v Heap, val: &serde_json::Value) -> Value<'v> {
    match val {
        serde_json::Value::Null => Value::new_none(),
        serde_json::Value::Bool(b) => Value::new_bool(*b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                heap.alloc(i as i32)
            } else {
                heap.alloc(n.as_f64().unwrap_or(0.0))
            }
        }
        serde_json::Value::String(s) => heap.alloc(s.as_str()),
        serde_json::Value::Array(arr) => {
            let items: Vec<Value<'v>> = arr.iter().map(|v| json_to_starlark(heap, v)).collect();
            heap.alloc(AllocList(items))
        }
        serde_json::Value::Object(obj) => {
            let entries: Vec<(&str, Value<'v>)> = obj
                .iter()
                .map(|(k, v)| (k.as_str(), json_to_starlark(heap, v)))
                .collect();
            heap.alloc(AllocDict(entries))
        }
    }
}

/// Convert a toml::Value to a Starlark Value
fn toml_to_starlark<'v>(heap: &'v Heap, val: &toml::Value) -> Value<'v> {
    match val {
        toml::Value::String(s) => heap.alloc(s.as_str()),
        toml::Value::Integer(i) => heap.alloc(*i as i32),
        toml::Value::Float(f) => heap.alloc(*f),
        toml::Value::Boolean(b) => Value::new_bool(*b),
        toml::Value::Datetime(dt) => heap.alloc(dt.to_string().as_str()),
        toml::Value::Array(arr) => {
            let items: Vec<Value<'v>> = arr.iter().map(|v| toml_to_starlark(heap, v)).collect();
            heap.alloc(AllocList(items))
        }
        toml::Value::Table(tbl) => {
            let entries: Vec<(&str, Value<'v>)> = tbl
                .iter()
                .map(|(k, v)| (k.as_str(), toml_to_starlark(heap, v)))
                .collect();
            heap.alloc(AllocDict(entries))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn workspace() -> PathBuf {
        env::current_dir().unwrap()
    }

    #[test]
    fn eval_simple_expression() {
        let result = evaluate(&workspace(), "1 + 2").unwrap();
        assert_eq!(result, "3");
    }

    #[test]
    fn eval_string_result() {
        let result = evaluate(&workspace(), r#""hello " + "world""#).unwrap();
        assert_eq!(result, "hello world");
    }

    #[test]
    fn eval_read_file() {
        let result = evaluate(&workspace(), r#"len(read_file("Cargo.toml")) > 0"#).unwrap();
        assert_eq!(result, "True");
    }

    #[test]
    fn eval_find_files() {
        let result = evaluate(&workspace(), r#"len(find_files("**/*.rs")) > 0"#).unwrap();
        assert_eq!(result, "True");
    }

    #[test]
    fn eval_lines_helper() {
        let result = evaluate(&workspace(), r#"len(lines("a\nb\nc"))"#).unwrap();
        assert_eq!(result, "3");
    }

    #[test]
    fn eval_json_parse() {
        let result = evaluate(
            &workspace(),
            r#"json_parse('{"name": "test", "count": 42}')["name"]"#,
        )
        .unwrap();
        assert_eq!(result, "test");
    }

    #[test]
    fn eval_toml_parse() {
        let result = evaluate(
            &workspace(),
            r#"toml_parse(read_file("Cargo.toml"))["package"]["name"]"#,
        )
        .unwrap();
        assert_eq!(result, "faqifai");
    }

    #[test]
    fn eval_regex_find() {
        let result = evaluate(
            &workspace(),
            r#"regex_find(r"[0-9]+", "foo 123 bar 456")"#,
        )
        .unwrap();
        assert_eq!(result, r#"["123", "456"]"#);
    }

    #[test]
    fn eval_regex_match() {
        let result = evaluate(&workspace(), r#"regex_match(r"fn\s+main", read_file("src/main.rs"))"#).unwrap();
        assert_eq!(result, "True");
    }

    #[test]
    fn eval_multiline_script() {
        let script = r#"
files = find_files("src/**/*.rs")
result = []
for f in files:
    if "main" in f:
        result.append(f)
len(result) > 0
"#;
        let result = evaluate(&workspace(), script).unwrap();
        assert_eq!(result, "True");
    }

    #[test]
    fn eval_sandbox_prevents_traversal() {
        let result = evaluate(&workspace(), r#"read_file("../../../etc/passwd")"#);
        assert!(result.is_err());
    }

    #[test]
    fn eval_f_string() {
        let result = evaluate(&workspace(), r#"
name = "world"
f"hello {name}"
"#).unwrap();
        assert_eq!(result, "hello world");
    }

    #[test]
    fn eval_join_pattern() {
        let result = evaluate(&workspace(), r#"
items = ["a", "b", "c"]
"\n".join(items)
"#).unwrap();
        assert_eq!(result, "a\nb\nc");
    }

    #[test]
    fn eval_dict_get_with_default() {
        let result = evaluate(&workspace(), r#"
d = json_parse('{"a": 1}')
str(d.get("b", "missing"))
"#).unwrap();
        assert_eq!(result, "missing");
    }

    #[test]
    fn eval_list_comprehension() {
        let result = evaluate(&workspace(), r#"
files = find_files("src/**/*.rs")
mods = [f for f in files if "mod" in f]
len(mods) >= 0
"#).unwrap();
        assert_eq!(result, "True");
    }

    #[test]
    fn eval_cargo_toml_analysis() {
        let result = evaluate(&workspace(), r#"
cargo = toml_parse(read_file("Cargo.toml"))
deps = cargo.get("dependencies", {})
results = []
for name in sorted(deps.keys()):
    spec = deps[name]
    if type(spec) == "dict" and "features" in spec:
        feats = str(spec["features"])
        results.append(name + ": features=" + feats)
    else:
        results.append(name + ": " + str(spec))
"\n".join(results)
"#).unwrap();
        assert!(result.contains("tokio:"));
        assert!(result.contains("serde:"));
    }

    #[test]
    fn eval_dict_items_unpacking() {
        let result = evaluate(&workspace(), r#"
d = {"a": 1, "b": 2}
results = []
for k, v in d.items():
    results.append(k + "=" + str(v))
",".join(sorted(results))
"#).unwrap();
        assert_eq!(result, "a=1,b=2");
    }

    #[test]
    fn eval_string_format_method() {
        let result = evaluate(&workspace(), r#"
name = "test"
count = 42
"{}: {} items".format(name, count)
"#).unwrap();
        assert_eq!(result, "test: 42 items");
    }
}
