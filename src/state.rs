use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use crate::codebase;
use crate::config;

/// Frontmatter stored in generated output markdown files
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OutputFrontmatter {
    pub generated_by: String,
    pub source: String,
    pub toc: Vec<TocEntry>,
}

/// Per-question entry in the TOC
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TocEntry {
    pub question: String,
    pub anchor: String,
    pub generated_at: DateTime<Utc>,
    pub sources: Vec<SourceFile>,
}

/// A source file with its hash at generation time
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SourceFile {
    pub path: String,
    pub sha256: String,
}

/// Staleness result for a single question
#[derive(Debug)]
pub enum Staleness {
    /// Answer is fresh (within TTL, no source changes)
    Fresh,
    /// Never generated
    NeverGenerated,
    /// TTL has expired
    TtlExpired,
    /// Source file(s) have changed
    SourcesChanged(Vec<String>),
}

/// Parse the frontmatter from an existing output markdown file
pub fn parse_output_frontmatter(output_path: &Path) -> Result<Option<OutputFrontmatter>> {
    if !output_path.exists() {
        return Ok(None);
    }

    let content = std::fs::read_to_string(output_path)?;
    let frontmatter = extract_frontmatter(&content)?;
    match frontmatter {
        Some(yaml_str) => {
            let fm: OutputFrontmatter = serde_yaml::from_str(&yaml_str)?;
            Ok(Some(fm))
        }
        None => Ok(None),
    }
}

/// Extract YAML frontmatter from markdown content (between --- delimiters)
fn extract_frontmatter(content: &str) -> Result<Option<String>> {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        return Ok(None);
    }

    let after_first = &trimmed[3..];
    if let Some(end_idx) = after_first.find("\n---") {
        let yaml = &after_first[..end_idx];
        Ok(Some(yaml.trim().to_string()))
    } else {
        Ok(None)
    }
}

/// Extract the answer body for a specific question from existing markdown
pub fn extract_answer(content: &str, question: &str) -> Option<String> {
    let heading = format!("# {}", question);
    let lines: Vec<&str> = content.lines().collect();

    let start = lines.iter().position(|l| l.trim() == heading)?;
    let body_start = start + 1;

    // Find the next heading or end of file
    let end = lines[body_start..]
        .iter()
        .position(|l| l.starts_with("# "))
        .map(|pos| body_start + pos)
        .unwrap_or(lines.len());

    let answer: String = lines[body_start..end].join("\n");
    Some(answer.trim().to_string())
}

/// Check staleness of a question against the existing output
pub fn check_staleness(
    root: &Path,
    output_path: &Path,
    question_text: &str,
    ttl: Option<&str>,
    cache: &mut std::collections::HashMap<String, String>,
) -> Result<Staleness> {
    let frontmatter = parse_output_frontmatter(output_path)?;

    let fm = match frontmatter {
        Some(fm) => fm,
        None => return Ok(Staleness::NeverGenerated),
    };

    // Find the TOC entry for this question
    let entry = fm.toc.iter().find(|e| e.question == question_text);
    let entry = match entry {
        Some(e) => e,
        None => return Ok(Staleness::NeverGenerated),
    };

    // Check TTL
    if let Some(ttl_str) = ttl {
        let duration = config::parse_ttl(ttl_str)?;
        let now = Utc::now();
        let age = now - entry.generated_at;
        if age > duration {
            return Ok(Staleness::TtlExpired);
        }
    }

    // Check source hashes
    let mut changed = Vec::new();
    for source in &entry.sources {
        match codebase::hash_source_cached(root, &source.path, cache) {
            Ok(current_hash) => {
                if current_hash != source.sha256 {
                    changed.push(source.path.clone());
                }
            }
            Err(_) => {
                // File was deleted or became unreadable
                changed.push(source.path.clone());
            }
        }
    }

    if !changed.is_empty() {
        return Ok(Staleness::SourcesChanged(changed));
    }

    Ok(Staleness::Fresh)
}

/// Build a map of question -> existing answer from an output file
pub fn load_existing_answers(output_path: &Path) -> Result<HashMap<String, String>> {
    let mut answers = HashMap::new();
    if !output_path.exists() {
        return Ok(answers);
    }

    let content = std::fs::read_to_string(output_path)?;
    let frontmatter = parse_output_frontmatter(output_path)?;

    if let Some(fm) = frontmatter {
        for entry in &fm.toc {
            if let Some(answer) = extract_answer(&content, &entry.question) {
                answers.insert(entry.question.clone(), answer);
            }
        }
    }

    Ok(answers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_frontmatter() {
        let content = "---\ngenerated_by: faqifai\nsource: test.faq\ntoc: []\n---\n\n# Question\n\nAnswer";
        let fm = extract_frontmatter(content).unwrap().unwrap();
        assert!(fm.contains("generated_by: faqifai"));
    }

    #[test]
    fn test_extract_frontmatter_none() {
        let content = "# No frontmatter\n\nJust content";
        assert!(extract_frontmatter(content).unwrap().is_none());
    }

    #[test]
    fn test_extract_answer() {
        let content = "---\nfm\n---\n\n# First question\n\nFirst answer here.\n\n# Second question\n\nSecond answer.";
        assert_eq!(
            extract_answer(content, "First question").unwrap(),
            "First answer here."
        );
        assert_eq!(
            extract_answer(content, "Second question").unwrap(),
            "Second answer."
        );
        assert!(extract_answer(content, "Missing").is_none());
    }
}
