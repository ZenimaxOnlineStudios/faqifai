//! Orchestrator: wires discovery → staleness → AI → output together.

use anyhow::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::ai::{self, AnswerResult, QuestionInput};
use crate::codebase;
use crate::discovery;
use crate::output;
use crate::state::{self, Staleness, TocEntry};

/// Run the FAQ generation pipeline
pub async fn run(root: &Path, scan_path: &Path, concurrency: usize, force: bool, model: &str) -> Result<()> {
    let discovered = discovery::discover(scan_path)?;
    if discovered.is_empty() {
        println!("No .faq files found.");
        return Ok(());
    }

    println!("Found {} .faq file(s)", discovered.len());

    // Shared hash cache — avoids rehashing the same file/glob for every question
    let mut hash_cache: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    // Group questions by output file
    let mut groups: HashMap<PathBuf, OutputGroup> = HashMap::new();

    for dfaq in &discovered {
        let faq_rel = dfaq
            .path
            .strip_prefix(root)
            .unwrap_or(&dfaq.path)
            .to_string_lossy()
            .replace('\\', "/");

        // All paths in a .faq file are relative to the .faq file's directory
        let faq_dir = dfaq.path.parent().unwrap_or(root);

        for question in &dfaq.faq.questions {
            let output_rel = dfaq.faq.output_for(question);
            let output_path = faq_dir.join(output_rel);

            let group = groups.entry(output_path.clone()).or_insert_with(|| OutputGroup {
                output_path: output_path.clone(),
                source_faq: faq_rel.clone(),
                questions: Vec::new(),
            });

            let ttl = dfaq.faq.ttl_for(question).map(|s| s.to_string());

            // Resolve scope relative to faq dir, then make relative to root
            let scope = dfaq.faq.scope_for(question).map(|s| {
                let resolved = faq_dir.join(s);
                resolved.strip_prefix(root)
                    .unwrap_or(&resolved)
                    .to_string_lossy()
                    .replace('\\', "/")
            });

            let context = dfaq.faq.context_for(question);

            // Resolve hint paths relative to faq dir, then make relative to root
            let hints = question.hints.as_ref().map(|hs| {
                hs.iter().map(|h| {
                    let resolved = faq_dir.join(h);
                    resolved.strip_prefix(root)
                        .unwrap_or(&resolved)
                        .to_string_lossy()
                        .replace('\\', "/")
                }).collect::<Vec<_>>()
            });

            let staleness = if force {
                Staleness::NeverGenerated
            } else {
                state::check_staleness(root, &output_path, &question.text, ttl.as_deref(), &mut hash_cache)?
            };

            group.questions.push(GroupedQuestion {
                text: question.text.clone(),
                context,
                hints,
                scope,
                staleness,
            });
        }
    }

    // Collect stale questions across all groups for concurrent processing
    let mut ai_inputs: Vec<(PathBuf, QuestionInput)> = Vec::new();

    for group in groups.values() {
        // Load existing answers so we can provide them as context for regeneration
        let existing_answers = state::load_existing_answers(&group.output_path)?;

        for q in &group.questions {
            if matches!(q.staleness, Staleness::Fresh) {
                continue;
            }

            let reason = match &q.staleness {
                Staleness::NeverGenerated => "never generated".to_string(),
                Staleness::TtlExpired => "TTL expired".to_string(),
                Staleness::SourcesChanged(files) => format!("sources changed: {}", files.join(", ")),
                Staleness::Fresh => unreachable!(),
            };
            eprintln!("⚡ Stale ({}): {}", reason, q.text);

            let codebase_ctx = codebase::collect_context(
                root,
                q.scope.as_deref(),
                q.hints.as_deref(),
                &mut hash_cache,
            )?;

            // Extract changed sources from staleness info
            let changed_sources = match &q.staleness {
                Staleness::SourcesChanged(files) => Some(files.clone()),
                _ => None,
            };

            // Get previous answer if one exists
            let previous_answer = existing_answers.get(&q.text).cloned();

            ai_inputs.push((
                group.output_path.clone(),
                QuestionInput {
                    question: q.text.clone(),
                    context: q.context.clone(),
                    hints: q.hints.clone(),
                    source_hashes: codebase_ctx.hashes,
                    output_path: group
                        .output_path
                        .strip_prefix(root)
                        .unwrap_or(&group.output_path)
                        .to_path_buf(),
                    previous_answer,
                    changed_sources,
                },
            ));
        }
    }

    if ai_inputs.is_empty() {
        println!("All questions are fresh. Nothing to do.");
        return Ok(());
    }

    println!(
        "Regenerating {} question(s) with concurrency={}",
        ai_inputs.len(),
        concurrency
    );

    // Extract just the QuestionInputs for the AI call
    let inputs: Vec<QuestionInput> = ai_inputs.iter().map(|(_, qi)| QuestionInput {
        question: qi.question.clone(),
        context: qi.context.clone(),
        hints: qi.hints.clone(),
        source_hashes: qi.source_hashes.clone(),
        output_path: qi.output_path.clone(),
        previous_answer: qi.previous_answer.clone(),
        changed_sources: qi.changed_sources.clone(),
    }).collect();

    let results = ai::answer_questions_concurrent(root, inputs, concurrency, model).await?;

    // Match results back to output groups
    let mut new_answers: HashMap<PathBuf, Vec<AnswerResult>> = HashMap::new();
    for (i, result) in results.into_iter().enumerate() {
        let (output_path, _) = &ai_inputs[i];
        match result {
            Ok(answer) => {
                new_answers
                    .entry(output_path.clone())
                    .or_default()
                    .push(answer);
            }
            Err(e) => {
                let (_, qi) = &ai_inputs[i];
                tracing::error!("Failed to answer '{}': {}", qi.question, e);
                eprintln!("ERROR: Failed to answer '{}': {}", qi.question, e);
            }
        }
    }

    // Write output files
    let mut files_written = 0;
    for (output_path, group) in &groups {
        let all_questions: Vec<String> = group.questions.iter().map(|q| q.text.clone()).collect();

        // Load existing answers for fresh questions
        let existing_answers = state::load_existing_answers(output_path)?;
        let existing_fm = state::parse_output_frontmatter(output_path)?;
        let existing_toc: Vec<TocEntry> = existing_fm.map(|fm| fm.toc).unwrap_or_default();

        // Merge existing (fresh) + new answers
        let mut merged_answers: HashMap<String, String> = HashMap::new();
        let mut merged_sources: HashMap<String, Vec<(String, String)>> = HashMap::new();

        // Keep fresh answers
        for q in &group.questions {
            if matches!(q.staleness, Staleness::Fresh) {
                if let Some(answer) = existing_answers.get(&q.text) {
                    merged_answers.insert(q.text.clone(), answer.clone());
                }
            }
        }

        // Add new answers
        if let Some(answers) = new_answers.get(output_path) {
            for answer in answers {
                merged_answers.insert(answer.question.clone(), answer.answer.clone());
                merged_sources.insert(answer.question.clone(), answer.sources.clone());
            }
        }

        output::write_output(
            output_path,
            &group.source_faq,
            &all_questions,
            &merged_answers,
            &merged_sources,
            &existing_toc,
        )?;

        files_written += 1;
    }

    println!("Done. Wrote {} output file(s).", files_written);
    Ok(())
}

/// Show status of all FAQ questions
pub fn status(root: &Path, scan_path: &Path) -> Result<()> {
    let discovered = discovery::discover(scan_path)?;
    if discovered.is_empty() {
        println!("No .faq files found.");
        return Ok(());
    }

    for dfaq in &discovered {
        let faq_rel = dfaq
            .path
            .strip_prefix(root)
            .unwrap_or(&dfaq.path)
            .to_string_lossy()
            .replace('\\', "/");

        let faq_dir = dfaq.path.parent().unwrap_or(root);

        println!("\n{}", faq_rel);
        println!("{}", "-".repeat(faq_rel.len()));

        for question in &dfaq.faq.questions {
            let output_rel = dfaq.faq.output_for(question);
            let output_path = faq_dir.join(output_rel);
            let ttl = dfaq.faq.ttl_for(question).map(|s| s.to_string());

            let staleness = state::check_staleness(
                root,
                &output_path,
                &question.text,
                ttl.as_deref(),
                &mut std::collections::HashMap::new(),
            )?;

            let status_str = match &staleness {
                Staleness::Fresh => "\x1b[32m✓\x1b[0m fresh".to_string(),
                Staleness::NeverGenerated => "○ never generated".to_string(),
                Staleness::TtlExpired => "⏰ TTL expired".to_string(),
                Staleness::SourcesChanged(files) => {
                    format!("△ {} source(s) changed", files.len())
                }
            };

            println!("  {} → {} [{}]", question.text, output_rel, status_str);
        }
    }

    Ok(())
}

/// List all questions (machine-readable)
pub fn list(root: &Path, scan_path: &Path, json: bool) -> Result<()> {
    let discovered = discovery::discover(scan_path)?;

    if json {
        let mut items: Vec<serde_json::Value> = Vec::new();
        for dfaq in &discovered {
            let faq_rel = dfaq
                .path
                .strip_prefix(root)
                .unwrap_or(&dfaq.path)
                .to_string_lossy()
                .replace('\\', "/");
            let faq_dir = dfaq.path.parent().unwrap_or(root);

            for question in &dfaq.faq.questions {
                let output_path = faq_dir.join(dfaq.faq.output_for(question));
                let output_rel = output_path.strip_prefix(root)
                    .unwrap_or(&output_path)
                    .to_string_lossy()
                    .replace('\\', "/");
                items.push(serde_json::json!({
                    "source": faq_rel,
                    "question": question.text,
                    "output": output_rel,
                }));
            }
        }
        println!("{}", serde_json::to_string_pretty(&items)?);
    } else {
        for dfaq in &discovered {
            let faq_dir = dfaq.path.parent().unwrap_or(root);
            for question in &dfaq.faq.questions {
                let output_path = faq_dir.join(dfaq.faq.output_for(question));
                let output_rel = output_path.strip_prefix(root)
                    .unwrap_or(&output_path)
                    .to_string_lossy()
                    .replace('\\', "/");
                println!("{}\t{}", question.text, output_rel);
            }
        }
    }

    Ok(())
}

/// Get answer for a specific question
pub fn get(root: &Path, scan_path: &Path, query: &str, raw: bool) -> Result<()> {
    let discovered = discovery::discover(scan_path)?;
    let query_lower = query.to_lowercase();

    for dfaq in &discovered {
        let faq_dir = dfaq.path.parent().unwrap_or(root);
        for question in &dfaq.faq.questions {
            if question.text.to_lowercase().contains(&query_lower) {
                let output_path = faq_dir.join(dfaq.faq.output_for(question));
                let output_rel = output_path.strip_prefix(root)
                    .unwrap_or(&output_path)
                    .to_string_lossy()
                    .replace('\\', "/");

                if !output_path.exists() {
                    if raw {
                        return Ok(());
                    }
                    println!("Question: {}", question.text);
                    println!("Status: not yet generated");
                    return Ok(());
                }

                let content = std::fs::read_to_string(&output_path)?;
                let answer = state::extract_answer(&content, &question.text);

                if raw {
                    if let Some(a) = answer {
                        print!("{}", a);
                    }
                } else {
                    println!("Question: {}", question.text);
                    println!("Output: {}", output_rel);
                    println!();
                    println!("{}", answer.unwrap_or_else(|| "No answer found.".to_string()));
                }
                return Ok(());
            }
        }
    }

    eprintln!("No question matching '{}' found.", query);
    std::process::exit(1);
}

/// Search across all generated answers
pub fn search(root: &Path, scan_path: &Path, pattern: &str, json: bool) -> Result<()> {
    let discovered = discovery::discover(scan_path)?;
    let pattern_lower = pattern.to_lowercase();
    let mut results: Vec<serde_json::Value> = Vec::new();

    for dfaq in &discovered {
        let faq_dir = dfaq.path.parent().unwrap_or(root);
        for question in &dfaq.faq.questions {
            let output_path = faq_dir.join(dfaq.faq.output_for(question));
            let output_rel = output_path.strip_prefix(root)
                .unwrap_or(&output_path)
                .to_string_lossy()
                .replace('\\', "/");

            if !output_path.exists() {
                continue;
            }

            let content = std::fs::read_to_string(&output_path)?;
            if let Some(answer) = state::extract_answer(&content, &question.text) {
                if answer.to_lowercase().contains(&pattern_lower)
                    || question.text.to_lowercase().contains(&pattern_lower)
                {
                    if json {
                        // Extract a snippet around the match
                        let snippet = extract_snippet(&answer, &pattern_lower, 200);
                        results.push(serde_json::json!({
                            "question": question.text,
                            "output": output_rel,
                            "snippet": snippet,
                        }));
                    } else {
                        println!("{}\t{}", question.text, output_rel);
                    }
                }
            }
        }
    }

    if json {
        println!("{}", serde_json::to_string_pretty(&results)?);
    }

    Ok(())
}

/// Extract a snippet of text around the first match
fn extract_snippet(text: &str, pattern: &str, max_len: usize) -> String {
    let lower = text.to_lowercase();
    if let Some(pos) = lower.find(pattern) {
        let start = pos.saturating_sub(max_len / 2);
        let end = (pos + pattern.len() + max_len / 2).min(text.len());
        let snippet = &text[start..end];
        if start > 0 { format!("...{}", snippet) } else { snippet.to_string() }
    } else {
        text.chars().take(max_len).collect()
    }
}

// Internal types

struct OutputGroup {
    output_path: PathBuf,
    source_faq: String,
    questions: Vec<GroupedQuestion>,
}

struct GroupedQuestion {
    text: String,
    context: Option<String>,
    hints: Option<Vec<String>>,
    scope: Option<String>,
    staleness: Staleness,
}
