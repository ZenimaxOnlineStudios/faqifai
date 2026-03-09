use anyhow::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::copilot::{
    Client, ClientOptions, Session, SessionConfig, SystemMessageConfig, ToolDefinition, ToolResult,
};
use crate::eval;
use crate::codebase;
use crate::state::{LineRange, SourceFile};

/// A source file path (and optional line range) recorded by the AI during a session
struct RecordedSource {
    path: String,
    start_line: Option<u32>,
    end_line: Option<u32>,
}

/// Shared state for tracking mark_unchanged signals within a session
struct SessionSharedState {
    current_question: Arc<std::sync::Mutex<Option<String>>>,
    unchanged_set: Arc<std::sync::Mutex<std::collections::HashSet<String>>>,
    recorded_sources: Arc<std::sync::Mutex<Vec<RecordedSource>>>,
}

impl SessionSharedState {
    fn new() -> Self {
        Self {
            current_question: Arc::new(std::sync::Mutex::new(None)),
            unchanged_set: Arc::new(std::sync::Mutex::new(std::collections::HashSet::new())),
            recorded_sources: Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }

    fn set_current(&self, question: &str) {
        *self.current_question.lock().unwrap() = Some(question.to_string());
        self.recorded_sources.lock().unwrap().clear();
    }

    fn is_unchanged(&self, question: &str) -> bool {
        self.unchanged_set.lock().unwrap().contains(question)
    }

    fn drain_sources(&self) -> Vec<RecordedSource> {
        std::mem::take(&mut *self.recorded_sources.lock().unwrap())
    }
}

/// System prompt: instructs the AI on how to answer codebase questions.
/// The `{output_dir}` placeholder is replaced at runtime with the output file's
/// directory relative to the repo root, so the AI can write correct relative paths.
const SYSTEM_PROMPT: &str = r#"You are a codebase research agent generating reference documentation that will be consumed by other AI agents and developers. Your output is cached and reused — thoroughness now saves repeated work later.

## Your mandate

1. **Be thorough.** Investigate the question fully. Trace call chains, follow imports, read config files. Surface the complete picture rather than a shallow summary. Include edge cases, error paths, and non-obvious behavior you discover.

2. **Be factual and unopinionated.** State what the code does, not what it should do. Do not suggest improvements, refactors, or alternatives unless the question explicitly asks for them. Never speculate — if something is unclear from the source, say so.

3. **Reference source files.** Every claim must be traceable. When describing behavior, cite the file where it is implemented using a path relative to the output file's location (the `{output_dir}` prefix handles this). ALWAYS use markdown links for file references — never bare filenames. The link text MUST always include the filename so the reader knows what file they are opening:
   - `[module_name]({output_dir}/src/auth/mod.rs)` for a file reference
   - `[auth/mod.rs:42]({output_dir}/src/auth/mod.rs#L42)` when a specific line matters
   - `[auth/mod.rs:42-55]({output_dir}/src/auth/mod.rs#L42-L55)` for a line range
   - WRONG: `jsonrpc.rs:34-48` or `src/auth/mod.rs` as plain text
   - WRONG: `[lines 34-48]({output_dir}/src/copilot/jsonrpc.rs#L34-L48)` — link text must contain the filename
   - WRONG: `[L34-L48]({output_dir}/src/copilot/jsonrpc.rs#L34-L48)` — link text must contain the filename
   - RIGHT: `[jsonrpc.rs:34-48]({output_dir}/src/copilot/jsonrpc.rs#L34-L48)`

4. **Use your tools aggressively.** You have file navigation and search tools — use them. Typical workflow:
   - Start by listing the root directory to understand project layout
   - Use search and file listing tools to locate relevant files
   - Use grep/search to find function names, types, imports, error messages, string literals
   - Read files to understand implementations, or request specific line ranges for large files
   - **Use `analyze` instead of making 3 or more sequential read/grep tool calls.** If you need to read multiple files, cross-reference data, aggregate results, or parse structured files (TOML, JSON), write an `analyze` script to do it in one call. This is faster and cheaper than chained tool calls.
   - Follow imports and references — trace the complete call chain before answering
   - **In a multi-question session, build on your prior research.** Files already read are in your context — don't re-read them unless you need a specific line range or the content may have changed.
   It is better to make too many tool calls than to guess.

5. **Structure for scanability.** Use sub-headings, bullet points, and code blocks. Lead with the direct answer, then expand with supporting detail. Keep prose minimal — prefer structured content over paragraphs.

6. **Output only the answer body.** Do not include the question as a heading (the tool adds headings). Do not add frontmatter, preamble, or sign-off. Start directly with the content. NEVER write sentences like "I have all the information I need", "Let me create the answer", "Perfect!", or any other meta-commentary — your first character of output must be part of the actual answer.

7. **When a previous answer is provided,** use it as a research accelerator — it tells you what was previously found and how the answer was structured. Verify its claims against the current source code, especially for any files flagged as changed. Keep what is still accurate, update what has changed, and remove anything no longer true. Do not mention that you are updating a previous answer. **If after thorough verification the previous answer is still fully accurate and complete, call the `mark_unchanged` tool (no arguments) and output nothing else.** Only use `mark_unchanged` when you are confident nothing has changed.

8. **Track your sources by calling `record_source`.** After reading any file that materially influenced your answer, call `record_source` with its path (relative to the workspace root). This is used to detect when the answer needs regenerating — be precise.
   - Use paths relative to the workspace root (e.g. `src/auth.rs`)
   - Use a trailing slash for directories when you want any new file added to that directory to trigger regeneration (e.g. `src/some/dir/`)
   - To track only a specific range of lines (e.g. one function), pass `start_line` and `end_line` — staleness checking will follow the content even if it moves to a different position
   - Include only files that materially influenced the answer — not every file you glanced at

## Starlark scripting reference (for the `analyze` tool)

The `analyze` tool executes a Starlark script and returns its last expression as a string. Starlark is Python-like but NOT Python. Use it for multi-step analysis that would otherwise require many sequential tool calls.

### Built-in functions

File I/O (all paths relative to workspace root):
- `read_file(path) -> str` — Read entire file contents.
- `read_file_lines(path, start, end) -> str` — Read line range (1-indexed, inclusive). Output has line numbers.
- `find_files(pattern) -> list[str]` — Find files by glob (e.g. "**/*.rs"). Returns list of paths.
- `list_dir(path) -> str` — List directory contents (one level). Use "" for root.
- `grep(pattern, glob="", max_results=50) -> str` — Regex search across files. Returns "path:line: text" lines.

Text processing:
- `lines(text) -> list[str]` — Split string into list of lines.
- `regex_find(pattern, text) -> list[str]` — Find all regex matches in text.
- `regex_match(pattern, text) -> bool` — Test if regex matches anywhere.

Data parsing:
- `json_parse(text) -> dict|list` — Parse JSON into Starlark dict/list.
- `toml_parse(text) -> dict` — Parse TOML into Starlark dict.

### Language notes (critical differences from Python)
- NO: import, class, try, except, while, set, with, yield, global, nonlocal. These do not exist in Starlark.
- Use `for x in range(n)` instead of `while`.
- Errors propagate immediately — check values before using them.
- f-strings support ONLY simple variable names: `f"value is {x}"`. For expressions use `+` and `str()`: `name + ": " + str(d["key"])`.
- Supports: for, if/elif/else, def, list comprehensions, dict comprehensions, lambda.
- Builtins: len, range, enumerate, zip, sorted, reversed, str, int, float, bool, list, dict, type, hasattr, getattr, repr, min, max, any, all.
- String methods: .split(), .strip(), .startswith(), .endswith(), .replace(), .upper(), .lower(), .find(), .count(), .join(), .format().
- List methods: .append(), .extend(), .insert(), .pop(), .remove(), .index().
- Dict methods: .get(key, default), .keys(), .values(), .items(), .pop(), .update().

### Examples

```starlark
# Cross-reference: find functions and where they're called
content = read_file("src/auth/mod.rs")
fns = regex_find(r"pub fn (\w+)", content)
results = []
for fn_name in fns:
    matches = grep(fn_name, glob="**/*.rs", max_results=10)
    count = len(lines(matches)) - 1
    results.append(fn_name + ": " + str(count) + " references")
"\n".join(results)
```

```starlark
# Parse Cargo.toml and list dependency details
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
```

```starlark
# Extract dependency names from package.json
pkg = json_parse(read_file("package.json"))
deps = list(pkg.get("dependencies", {}).keys())
dev = list(pkg.get("devDependencies", {}).keys())
"\n".join(["Dependencies:"] + sorted(deps) + ["", "Dev:"] + sorted(dev))
```"#;

/// Result of an AI-answered question
#[derive(Debug)]
pub struct AnswerResult {
    pub question: String,
    pub answer: String,
    pub sources: Vec<SourceFile>,
}




/// E.g., if output is "docs/api/auth.md", returns "../../" so that
/// `../../src/main.rs` is a valid relative link from the output file.
fn relative_prefix_to_root(output_path: &Path) -> String {
    let parent = output_path.parent().unwrap_or(Path::new(""));
    let depth = parent.components().count();
    if depth == 0 {
        "./".to_string()
    } else {
        "../".repeat(depth)
    }
}

/// Build the system prompt with the output directory's relative prefix baked in
fn build_system_prompt(output_path: &Path) -> String {
    let prefix = relative_prefix_to_root(output_path);
    SYSTEM_PROMPT.replace("{output_dir}", prefix.trim_end_matches('/'))
}

/// Build the user message sent to the AI for a single question
fn build_user_message(
    question: &str,
    context: Option<&str>,
    hints: Option<&[String]>,
    previous_answer: Option<&str>,
    changed_sources: Option<&[String]>,
    previous_sources: Option<&[String]>,
) -> String {
    let mut msg = String::new();

    if let Some(ctx) = context {
        msg.push_str("## Context\n\n");
        msg.push_str(ctx);
        msg.push_str("\n\n");
    }

    if let Some(hint_paths) = hints {
        if !hint_paths.is_empty() {
            msg.push_str("## Suggested Starting Points\n\n");
            for h in hint_paths {
                msg.push_str(&format!("- `{}`\n", h));
            }
            msg.push_str("\n");
        }
    }

    // When regenerating, provide the previous answer and what changed
    if let Some(prev) = previous_answer {
        if !prev.is_empty() {
            msg.push_str("## Previous Answer (for reference only)\n\n");
            msg.push_str("The following is a previous answer to this question. It may be partially or fully outdated. Use it to guide your research and help cross-check what is still correct, but do NOT trust it — verify every claim against the current source code.\n\n");

            if let Some(changes) = changed_sources {
                if !changes.is_empty() {
                    msg.push_str("**Detected changes since last generation:**\n");
                    for c in changes {
                        msg.push_str(&format!("- `{}`\n", c));
                    }
                    msg.push_str("\nPay special attention to these changed sources — they are likely why the answer needs updating.\n\n");
                }
            }

            if let Some(srcs) = previous_sources {
                if !srcs.is_empty() {
                    msg.push_str("**Files previously tracked as relevant** (call `record_source` for each that remains relevant — add new findings, omit any that no longer apply):\n");
                    for s in srcs {
                        msg.push_str(&format!("- `{}`\n", s));
                    }
                    msg.push_str("\n");
                }
            }

            msg.push_str("<previous_answer>\n");
            msg.push_str(prev);
            msg.push_str("\n</previous_answer>\n\n");
        }
    }

    msg.push_str("## Question\n\n");
    msg.push_str(question);

    msg
}

/// Custom tool definitions (only tools not built into copilot CLI)
fn tool_definitions() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition {
            name: "analyze".to_string(),
            description: Some("Run a Starlark script for multi-step codebase analysis. Use this instead of making multiple sequential read/grep calls — read many files, cross-reference data, parse TOML/JSON, and aggregate results all in one script. Returns the last expression as a string. See the Starlark scripting reference in your instructions for available functions, language rules, and examples.".to_string()),
            parameters: Some(serde_json::json!({
                "type": "object",
                "properties": {
                    "intent": {
                        "type": "string",
                        "description": "One-line description of what this script is doing, e.g. 'Find all public functions in src/auth/'."
                    },
                    "script": {
                        "type": "string",
                        "description": "Starlark script. The last expression is the return value."
                    }
                },
                "required": ["intent", "script"]
            })),
            overrides_built_in_tool: None,
        },
        ToolDefinition {
            name: "mark_unchanged".to_string(),
            description: Some("Signal that the previous answer is still fully accurate after verification. Call this (with no arguments) instead of rewriting the answer. Only use when you have verified all source files and are confident nothing has changed.".to_string()),
            parameters: Some(serde_json::json!({
                "type": "object",
                "properties": {}
            })),
            overrides_built_in_tool: None,
        },
        ToolDefinition {
            name: "record_source".to_string(),
            description: Some("Record a source file (or a specific line range within a file) as relevant to the current answer. Call this for every file you read that materially influenced the answer. Staleness detection uses these records to decide when to regenerate the answer.".to_string()),
            parameters: Some(serde_json::json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "File path relative to workspace root (e.g. \"src/auth.rs\"). Use a trailing slash for directories."
                    },
                    "start_line": {
                        "type": "integer",
                        "description": "First line of the relevant range (1-indexed, inclusive). Omit to track the whole file."
                    },
                    "end_line": {
                        "type": "integer",
                        "description": "Last line of the relevant range (1-indexed, inclusive). Required if start_line is provided."
                    }
                },
                "required": ["path"]
            })),
            overrides_built_in_tool: None,
        },
    ]
}

/// Built-in copilot tools to exclude (write capabilities we never want)
fn excluded_tools() -> Vec<String> {
    [
        "edit_file",
        "create_file",
        "delete_file",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
}

/// Answer a single question using the Copilot SDK
#[allow(dead_code)]
pub async fn answer_question(
    root: &Path,
    input: &QuestionInput,
    client: &Client,
    model: &str,
) -> Result<AnswerResult> {
    eprintln!("▶ Answering: {}", input.question);
    let system_prompt = build_system_prompt(&input.output_path);
    let previous_source_paths: Vec<String> = input.source_hashes.keys().cloned().collect();
    let user_message = build_user_message(
        &input.question,
        input.context.as_deref(),
        input.hints.as_deref(),
        input.previous_answer.as_deref(),
        input.changed_sources.as_deref(),
        if input.previous_answer.is_some() && !previous_source_paths.is_empty() {
            Some(&previous_source_paths)
        } else {
            None
        },
    );

    // Source hashes for staleness tracking (computed from scope/hints before the session)
    let sources = input.source_hashes.clone();

    let working_dir = root.to_string_lossy().replace('\\', "/");

    let shared = SessionSharedState::new();

    // Create a session with read-only built-in tools + our custom eval tool
    let session = client
        .create_session(SessionConfig {
            model: Some(model.to_string()),
            system_message: Some(SystemMessageConfig {
                mode: "replace".to_string(),
                content: system_prompt,
            }),
            tools: Some(tool_definitions()),
            excluded_tools: Some(excluded_tools()),
            working_directory: Some(working_dir),
            ..Default::default()
        })
        .await?;

    // Register custom tool handlers (only analyze — read tools are built into copilot)
    register_tools(&session, root, &shared).await;

    shared.set_current(&input.question);

    // Send the question and wait for the complete response
    let answer = session.send_and_wait(&user_message).await?;

    // Clean up the session
    if let Err(e) = session.destroy().await {
        tracing::warn!("Failed to destroy session: {}", e);
    }

    // If the AI called mark_unchanged, reuse the previous answer
    let answer = if shared.is_unchanged(&input.question) {
        if let Some(prev) = &input.previous_answer {
            eprintln!("\x1b[32m✓\x1b[0m No update needed: {}", input.question);
            prev.clone()
        } else {
            tracing::warn!("AI called mark_unchanged but no previous answer exists for '{}'; treating as empty", input.question);
            answer
        }
    } else {
        eprintln!("\x1b[32m✓\x1b[0m Answered ({} bytes): {}", answer.len(), input.question);
        answer
    };

    // Extract AI-reported sources from the answer block and use them for staleness tracking.
    // Fall back to the pre-computed scope/hints hashes if the AI didn't report sources.
    let recorded = shared.drain_sources();
    let final_sources = resolve_recorded_sources(root, recorded, sources);

    Ok(AnswerResult {
        question: input.question.clone(),
        answer,
        sources: final_sources,
    })
}

/// Register custom tool handlers on a session (only analyze — read tools are built-in)
async fn register_tools(
    session: &Session,
    root: &Path,
    shared: &SessionSharedState,
) {
    let r = root.to_path_buf();
    session
        .register_tool(
            "analyze",
            Arc::new(move |args| {
                let script = arg_str(&args, "script");
                match eval::evaluate(&r, &script) {
                    Ok(result) => ToolResult::success(result),
                    Err(e) => ToolResult::failure(e.to_string()),
                }
            }),
        )
        .await;

    let unchanged_set = shared.unchanged_set.clone();
    let current_question = shared.current_question.clone();
    session
        .register_tool(
            "mark_unchanged",
            Arc::new(move |_args| {
                if let Some(ref q) = *current_question.lock().unwrap() {
                    unchanged_set.lock().unwrap().insert(q.clone());
                }
                ToolResult::success("Marked as unchanged.".to_string())
            }),
        )
        .await;

    let recorded_sources = shared.recorded_sources.clone();
    session
        .register_tool(
            "record_source",
            Arc::new(move |args| {
                let path = arg_str(&args, "path");
                if path.is_empty() {
                    return ToolResult::failure("path is required".to_string());
                }
                let start_line = arg_u32(&args, "start_line");
                let end_line = arg_u32(&args, "end_line");
                recorded_sources.lock().unwrap().push(RecordedSource {
                    path,
                    start_line,
                    end_line,
                });
                ToolResult::success("Source recorded.".to_string())
            }),
        )
        .await;
}

/// Extract a string argument with a default of ""
fn arg_str(args: &serde_json::Value, key: &str) -> String {
    args.get(key)
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string()
}

/// Extract an optional u32 argument
fn arg_u32(args: &serde_json::Value, key: &str) -> Option<u32> {
    args.get(key).and_then(|v| v.as_u64()).map(|v| v as u32)
}

/// Hash recorded sources into `SourceFile` entries for staleness tracking.
/// Falls back to pre-computed `fallback` hashes if no sources were recorded.
fn resolve_recorded_sources(
    root: &Path,
    mut recorded: Vec<RecordedSource>,
    fallback: HashMap<String, String>,
) -> Vec<SourceFile> {
    if recorded.is_empty() {
        let mut sources: Vec<SourceFile> = fallback
            .into_iter()
            .map(|(path, sha256)| SourceFile { path, sha256, lines: None })
            .collect();
        sources.sort_by(|a, b| a.path.cmp(&b.path));
        return sources;
    }

    // Deduplicate by (path, start_line, end_line) — keep last occurrence
    recorded.reverse();
    let mut seen = std::collections::HashSet::new();
    recorded.retain(|r| seen.insert((r.path.clone(), r.start_line, r.end_line)));
    recorded.reverse();

    let mut sources = Vec::new();
    for rec in &recorded {
        if let (Some(start), Some(end)) = (rec.start_line, rec.end_line) {
            match codebase::hash_file_lines(root, &rec.path, start, end) {
                Ok((hash, content_len)) => sources.push(SourceFile {
                    path: rec.path.clone(),
                    sha256: hash,
                    lines: Some(LineRange { start, end, content_len }),
                }),
                Err(e) => tracing::warn!("Could not hash line range for '{}': {}", rec.path, e),
            }
        } else {
            match codebase::hash_source(root, &rec.path) {
                Ok(hash) => sources.push(SourceFile {
                    path: rec.path.clone(),
                    sha256: hash,
                    lines: None,
                }),
                Err(e) => tracing::warn!("Could not hash source '{}': {}", rec.path, e),
            }
        }
    }
    sources.sort_by(|a, b| a.path.cmp(&b.path));
    sources
}

/// Input for a single question to be answered concurrently
pub struct QuestionInput {
    pub question: String,
    pub context: Option<String>,
    pub hints: Option<Vec<String>>,
    pub source_hashes: HashMap<String, String>,
    pub output_path: PathBuf,
    /// The previous answer (if regenerating a stale answer)
    pub previous_answer: Option<String>,
    /// Sources that triggered regeneration (changed files/dirs/globs)
    pub changed_sources: Option<Vec<String>>,
}

/// Answer multiple questions sequentially in a single session.
/// All inputs must share the same output_path.
/// The session retains file context across questions — loaded files stay in context.
async fn answer_questions_in_session(
    root: &Path,
    inputs: &[QuestionInput],
    client: &Client,
    model: &str,
) -> Vec<Result<AnswerResult>> {
    // All questions share the same output_path
    let output_path = &inputs[0].output_path;
    let system_prompt = build_system_prompt(output_path);
    let working_dir = root.to_string_lossy().replace('\\', "/");

    let shared = SessionSharedState::new();

    let session = match client
        .create_session(SessionConfig {
            model: Some(model.to_string()),
            system_message: Some(SystemMessageConfig {
                mode: "replace".to_string(),
                content: system_prompt,
            }),
            tools: Some(tool_definitions()),
            excluded_tools: Some(excluded_tools()),
            working_directory: Some(working_dir),
            ..Default::default()
        })
        .await
    {
        Ok(s) => s,
        Err(e) => {
            return inputs
                .iter()
                .map(|_| Err(anyhow::anyhow!("Session creation failed: {}", e)))
                .collect()
        }
    };

    register_tools(&session, root, &shared).await;

    let mut results = Vec::new();

    for input in inputs {
        shared.set_current(&input.question);
        eprintln!("▶ Answering: {}", input.question);

        let previous_source_paths: Vec<String> = input.source_hashes.keys().cloned().collect();
        let user_message = build_user_message(
            &input.question,
            input.context.as_deref(),
            input.hints.as_deref(),
            input.previous_answer.as_deref(),
            input.changed_sources.as_deref(),
            if input.previous_answer.is_some() && !previous_source_paths.is_empty() {
                Some(&previous_source_paths)
            } else {
                None
            },
        );

        let result = match session.send_and_wait(&user_message).await {
            Ok(raw_answer) => {
                let answer = if shared.is_unchanged(&input.question) {
                    if let Some(prev) = &input.previous_answer {
                        eprintln!("\x1b[32m✓\x1b[0m No update needed: {}", input.question);
                        prev.clone()
                    } else {
                        tracing::warn!(
                            "AI called mark_unchanged but no previous answer for '{}'; treating as empty",
                            input.question
                        );
                        raw_answer
                    }
                } else {
                    eprintln!("\x1b[32m✓\x1b[0m Answered ({} bytes): {}", raw_answer.len(), input.question);
                    raw_answer
                };

                let recorded = shared.drain_sources();
                let sources = resolve_recorded_sources(root, recorded, input.source_hashes.clone());

                Ok(AnswerResult {
                    question: input.question.clone(),
                    answer,
                    sources,
                })
            }
            Err(e) => Err(e),
        };

        results.push(result);
    }

    if let Err(e) = session.destroy().await {
        tracing::warn!("Failed to destroy session: {}", e);
    }

    results
}

/// Answer multiple questions using a pool of worker tasks.
/// Questions sharing the same output file are answered in a single session so
/// the AI can reuse file context across questions. Workers process groups
/// concurrently — one session per group, up to `concurrency` sessions at once.
pub async fn answer_questions_concurrent(
    root: &Path,
    questions: Vec<QuestionInput>,
    concurrency: usize,
    model: &str,
) -> Result<Vec<Result<AnswerResult>>> {
    let client = Arc::new(
        Client::new(ClientOptions {
            working_directory: Some(root.to_path_buf()),
            ..Default::default()
        })
        .await?,
    );
    let root = root.to_path_buf();
    let model = model.to_string();
    let total = questions.len();

    // Group questions by output_path, preserving original indices for result ordering
    let mut groups: std::collections::HashMap<PathBuf, Vec<(usize, QuestionInput)>> =
        std::collections::HashMap::new();
    for (i, q) in questions.into_iter().enumerate() {
        groups.entry(q.output_path.clone()).or_default().push((i, q));
    }
    // Sort each group by original index so questions are answered in declaration order
    for group in groups.values_mut() {
        group.sort_by_key(|(i, _)| *i);
    }

    // Build work queue of groups
    let queue: Arc<Mutex<Vec<Vec<(usize, QuestionInput)>>>> =
        Arc::new(Mutex::new(groups.into_values().collect()));

    let results: Arc<Mutex<Vec<(usize, Result<AnswerResult>)>>> =
        Arc::new(Mutex::new(Vec::with_capacity(total)));

    // Spawn min(concurrency, num_groups) workers, each processing one group at a time
    let num_groups = { queue.lock().await.len() };
    let workers = concurrency.min(num_groups);
    let mut handles = Vec::new();

    for worker_id in 0..workers {
        let queue = queue.clone();
        let results = results.clone();
        let root = root.clone();
        let client = client.clone();
        let model = model.clone();

        handles.push(tokio::spawn(async move {
            loop {
                let group = { queue.lock().await.pop() };
                let group = match group {
                    Some(g) => g,
                    None => break,
                };

                tracing::debug!(
                    "Worker {} processing group of {} question(s)",
                    worker_id,
                    group.len()
                );

                let indices: Vec<usize> = group.iter().map(|(i, _)| *i).collect();
                let inputs: Vec<QuestionInput> =
                    group.into_iter().map(|(_, q)| q).collect();

                let group_results =
                    answer_questions_in_session(&root, &inputs, &client, &model).await;

                let mut locked = results.lock().await;
                for (idx, result) in indices.into_iter().zip(group_results.into_iter()) {
                    locked.push((idx, result));
                }
            }
        }));
    }

    for handle in handles {
        if let Err(e) = handle.await {
            tracing::error!("Worker panicked: {}", e);
        }
    }

    // Sort results back to original question order
    let mut results = Arc::try_unwrap(results)
        .map_err(|_| anyhow::anyhow!("Failed to unwrap results"))?
        .into_inner();
    results.sort_by_key(|(idx, _)| *idx);

    Ok(results.into_iter().map(|(_, r)| r).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relative_prefix_to_root() {
        assert_eq!(relative_prefix_to_root(Path::new("output.md")), "./");
        assert_eq!(relative_prefix_to_root(Path::new("docs/faq.md")), "../");
        assert_eq!(
            relative_prefix_to_root(Path::new("docs/api/auth.md")),
            "../../"
        );
    }

    #[test]
    fn test_system_prompt_has_no_raw_placeholder() {
        let prompt = build_system_prompt(Path::new("docs/out.md"));
        assert!(!prompt.contains("{output_dir}"));
        assert!(prompt.contains("../src/auth/mod.rs"));
    }
}
