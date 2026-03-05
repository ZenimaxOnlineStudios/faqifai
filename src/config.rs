use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Top-level .faq file structure (TOML)
#[derive(Debug, Deserialize, Serialize)]
pub struct FaqFile {
    /// Default TTL for all questions (e.g., "7d", "24h")
    pub ttl: Option<String>,
    /// Default scope glob pattern (e.g., "src/auth/**")
    pub scope: Option<String>,
    /// Default output markdown file path
    pub output: Option<String>,
    /// Context prompt applied to all questions
    pub context: Option<String>,
    /// List of questions
    #[serde(rename = "question")]
    pub questions: Vec<Question>,
}

/// A single question within a .faq file
#[derive(Debug, Deserialize, Serialize)]
pub struct Question {
    /// The question text
    pub text: String,
    /// Per-question context (appended to top-level context)
    pub context: Option<String>,
    /// Hint files the AI should prioritize
    pub hints: Option<Vec<String>>,
    /// Per-question output file (overrides top-level)
    pub output: Option<String>,
    /// Per-question TTL (overrides top-level)
    pub ttl: Option<String>,
    /// Per-question scope (overrides top-level)
    pub scope: Option<String>,
}

impl FaqFile {
    /// Load and parse a .faq TOML file
    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let faq: FaqFile = toml::from_str(&content)?;
        faq.validate(path)?;
        Ok(faq)
    }

    /// Validate that questions have an output target (top-level or per-question)
    fn validate(&self, path: &Path) -> Result<()> {
        for (i, q) in self.questions.iter().enumerate() {
            if self.output.is_none() && q.output.is_none() {
                anyhow::bail!(
                    "{}:question[{}]: no output specified (set top-level 'output' or per-question 'output')",
                    path.display(),
                    i
                );
            }
        }
        Ok(())
    }

    /// Get the resolved output path for a question
    pub fn output_for<'a>(&'a self, question: &'a Question) -> &'a str {
        question.output.as_deref().unwrap_or_else(|| self.output.as_deref().unwrap())
    }

    /// Get the resolved TTL for a question
    pub fn ttl_for<'a>(&'a self, question: &'a Question) -> Option<&'a str> {
        question.ttl.as_deref().or(self.ttl.as_deref())
    }

    /// Get the resolved scope for a question
    pub fn scope_for<'a>(&'a self, question: &'a Question) -> Option<&'a str> {
        question.scope.as_deref().or(self.scope.as_deref())
    }

    /// Get the combined context for a question
    pub fn context_for(&self, question: &Question) -> Option<String> {
        match (&self.context, &question.context) {
            (Some(top), Some(per)) => Some(format!("{}\n\n{}", top, per)),
            (Some(top), None) => Some(top.clone()),
            (None, Some(per)) => Some(per.clone()),
            (None, None) => None,
        }
    }
}

/// Parse a TTL string (e.g., "7d", "24h", "30m") into a chrono::Duration
pub fn parse_ttl(ttl: &str) -> Result<chrono::Duration> {
    let ttl = ttl.trim();
    if ttl.is_empty() {
        anyhow::bail!("empty TTL string");
    }
    let (num_str, unit) = ttl.split_at(ttl.len() - 1);
    let num: i64 = num_str.parse().map_err(|_| anyhow::anyhow!("invalid TTL number: {}", num_str))?;
    match unit {
        "d" => Ok(chrono::Duration::days(num)),
        "h" => Ok(chrono::Duration::hours(num)),
        "m" => Ok(chrono::Duration::minutes(num)),
        _ => anyhow::bail!("unknown TTL unit '{}' (expected d/h/m)", unit),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_basic_faq() {
        let toml_str = r#"
            ttl = "7d"
            scope = "src/**"
            output = "docs/faq.md"
            context = "A Rust project"

            [[question]]
            text = "How does it work?"

            [[question]]
            text = "What about testing?"
            output = "docs/testing.md"
            ttl = "3d"
            context = "Focus on unit tests"
            hints = ["src/tests/mod.rs"]
        "#;
        let faq: FaqFile = toml::from_str(toml_str).unwrap();
        assert_eq!(faq.questions.len(), 2);
        assert_eq!(faq.output_for(&faq.questions[0]), "docs/faq.md");
        assert_eq!(faq.output_for(&faq.questions[1]), "docs/testing.md");
        assert_eq!(faq.ttl_for(&faq.questions[0]), Some("7d"));
        assert_eq!(faq.ttl_for(&faq.questions[1]), Some("3d"));
        assert_eq!(
            faq.context_for(&faq.questions[1]).unwrap(),
            "A Rust project\n\nFocus on unit tests"
        );
    }

    #[test]
    fn parse_ttl_values() {
        assert_eq!(parse_ttl("7d").unwrap(), chrono::Duration::days(7));
        assert_eq!(parse_ttl("24h").unwrap(), chrono::Duration::hours(24));
        assert_eq!(parse_ttl("30m").unwrap(), chrono::Duration::minutes(30));
        assert!(parse_ttl("bad").is_err());
        assert!(parse_ttl("").is_err());
    }

    #[test]
    fn validate_missing_output() {
        let toml_str = r#"
            [[question]]
            text = "No output anywhere"
        "#;
        let faq: FaqFile = toml::from_str(toml_str).unwrap();
        assert!(faq.validate(Path::new("test.faq")).is_err());
    }
}
