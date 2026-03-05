use anyhow::Result;
use chrono::Utc;
use std::collections::HashMap;
use std::path::Path;

use crate::state::{OutputFrontmatter, SourceFile, TocEntry};

/// Generate an anchor from a question heading
fn question_to_anchor(question: &str) -> String {
    let slug: String = question
        .to_lowercase()
        .chars()
        .filter(|c| *c != '\'')
        .map(|c| if c.is_alphanumeric() || c == ' ' { c } else { ' ' })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("-");
    format!("#{}", slug)
}

/// Build or update an output markdown file
///
/// Takes existing answers (for fresh questions) and new answers (for regenerated questions),
/// merges them, and writes the complete output file with frontmatter.
pub fn write_output(
    output_path: &Path,
    source_faq_path: &str,
    questions: &[String],
    answers: &HashMap<String, String>,
    sources: &HashMap<String, Vec<(String, String)>>,
    existing_toc: &[TocEntry],
) -> Result<()> {
    let now = Utc::now();

    // Build TOC entries
    let mut toc = Vec::new();
    for question in questions {
        let anchor = question_to_anchor(question);

        // Use existing entry's metadata if the answer wasn't regenerated
        let entry = if let Some(existing) = existing_toc.iter().find(|e| e.question == *question) {
            if sources.contains_key(question) {
                // Regenerated — use new metadata
                TocEntry {
                    question: question.clone(),
                    anchor,
                    generated_at: now,
                    sources: sources[question]
                        .iter()
                        .map(|(path, hash)| SourceFile {
                            path: path.clone(),
                            sha256: hash.clone(),
                        })
                        .collect(),
                }
            } else {
                // Fresh — keep existing metadata
                existing.clone()
            }
        } else {
            // New question
            TocEntry {
                question: question.clone(),
                anchor,
                generated_at: now,
                sources: sources
                    .get(question)
                    .map(|s| {
                        s.iter()
                            .map(|(path, hash)| SourceFile {
                                path: path.clone(),
                                sha256: hash.clone(),
                            })
                            .collect()
                    })
                    .unwrap_or_default(),
            }
        };

        toc.push(entry);
    }

    let frontmatter = OutputFrontmatter {
        generated_by: "faqifai".to_string(),
        source: source_faq_path.to_string(),
        toc,
    };

    // Serialize frontmatter
    let fm_yaml = serde_yaml::to_string(&frontmatter)?;

    // Build markdown body
    let mut body = String::new();

    // Human-readable TOC
    body.push_str("## Contents\n\n");
    for entry in &frontmatter.toc {
        body.push_str(&format!("- [{}]({})\n", entry.question, entry.anchor));
    }
    body.push_str("\n---\n\n");

    for question in questions {
        body.push_str(&format!("# {}\n\n", question));
        if let Some(answer) = answers.get(question) {
            body.push_str(answer);
            body.push_str("\n\n");
        } else {
            body.push_str("*No answer generated.*\n\n");
        }
    }

    // Assemble final document
    let document = format!("---\n{}---\n\n{}", fm_yaml, body.trim_end());

    // Ensure parent directory exists
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::write(output_path, document)?;
    tracing::info!("Wrote output: {}", output_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_question_to_anchor() {
        assert_eq!(
            question_to_anchor("How does authentication work?"),
            "#how-does-authentication-work"
        );
        assert_eq!(
            question_to_anchor("What's the API?"),
            "#whats-the-api"
        );
    }
}
