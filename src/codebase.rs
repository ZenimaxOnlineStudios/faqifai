use anyhow::Result;
use glob::Pattern;
use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Source hashes collected from the codebase for staleness tracking.
/// Keys may be file paths, "dir/" directories, or glob patterns.
/// Directories and globs use a merkle tree hash of their contents.
#[derive(Debug, Clone)]
pub struct CodebaseContext {
    pub hashes: HashMap<String, String>,
}

impl CodebaseContext {
    pub fn new() -> Self {
        Self {
            hashes: HashMap::new(),
        }
    }

    /// Track a grouped source hash (directory merkle or glob merkle)
    fn add_source_hash(&mut self, key: String, hash: String) {
        self.hashes.insert(key, hash);
    }
}

/// Detect if a string contains glob metacharacters
pub fn is_glob_pattern(s: &str) -> bool {
    s.contains('*') || s.contains('?') || s.contains('[') || s.contains('{')
}

/// Compute a merkle tree hash from (path, content_hash) pairs.
/// Entries are sorted by path, then each "path\0hash\n" is fed into SHA-256.
/// Detects content changes, file additions, and file deletions.
pub fn merkle_hash(entries: &[(String, String)]) -> String {
    let mut sorted = entries.to_vec();
    sorted.sort_by(|a, b| a.0.cmp(&b.0));
    let mut hasher = Sha256::new();
    for (path, hash) in &sorted {
        hasher.update(path.as_bytes());
        hasher.update(b"\0");
        hasher.update(hash.as_bytes());
        hasher.update(b"\n");
    }
    format!("{:x}", hasher.finalize())
}

/// Collected file path and hash (internal helper)
struct FileEntry {
    rel_path: String,
    hash: String,
}

/// Walk the repository collecting files that match a glob pattern
fn walk_glob(root: &Path, pattern: &str) -> Result<Vec<FileEntry>> {
    let glob_pattern = Pattern::new(pattern)?;
    let canonical_root = root.canonicalize()?;
    let mut paths = Vec::new();

    for entry in ignore::WalkBuilder::new(root)
        .hidden(true)
        .git_ignore(true)
        .follow_links(false)
        .build()
    {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() { continue; }
        if let Ok(canonical) = path.canonicalize() {
            if !canonical.starts_with(&canonical_root) { continue; }
        }
        let relative = path.strip_prefix(root).unwrap_or(path);
        let rel_str = relative.to_string_lossy().replace('\\', "/");
        if glob_pattern.matches(&rel_str) {
            paths.push((rel_str, path.to_path_buf()));
        }
    }

    let entries: Vec<FileEntry> = paths
        .into_par_iter()
        .filter_map(|(rel_str, path)| {
            std::fs::read_to_string(&path).ok()
                .map(|content| FileEntry { rel_path: rel_str, hash: sha256_hex(&content) })
        })
        .collect();

    Ok(entries)
}

/// Walk a directory collecting all files under it
fn walk_dir(root: &Path, dir: &Path) -> Result<Vec<FileEntry>> {
    let canonical_root = root.canonicalize()?;
    let mut paths = Vec::new();

    for entry in ignore::WalkBuilder::new(dir)
        .hidden(true)
        .git_ignore(true)
        .follow_links(false)
        .build()
    {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() { continue; }
        if let Ok(canonical) = path.canonicalize() {
            if !canonical.starts_with(&canonical_root) { continue; }
        }
        let relative = path.strip_prefix(root).unwrap_or(path);
        let rel_str = relative.to_string_lossy().replace('\\', "/");
        paths.push((rel_str, path.to_path_buf()));
    }

    let entries: Vec<FileEntry> = paths
        .into_par_iter()
        .filter_map(|(rel_str, path)| {
            std::fs::read_to_string(&path).ok()
                .map(|content| FileEntry { rel_path: rel_str, hash: sha256_hex(&content) })
        })
        .collect();

    Ok(entries)
}

/// Collect source hashes for staleness tracking.
/// Scope and directory/glob hints produce merkle tree hashes; file hints produce individual hashes.
/// No file content is sent to the AI — it uses its own tools to explore.
pub fn collect_context(
    root: &Path,
    scope: Option<&str>,
    hints: Option<&[String]>,
    cache: &mut std::collections::HashMap<String, String>,
) -> Result<CodebaseContext> {
    let mut ctx = CodebaseContext::new();

    // Hash hint sources (files, directories, or glob patterns)
    if let Some(hint_paths) = hints {
        for hint in hint_paths {
            if is_glob_pattern(hint) {
                let entries = walk_glob(root, hint)?;
                let hash_pairs: Vec<_> = entries.iter()
                    .map(|e| (e.rel_path.clone(), e.hash.clone()))
                    .collect();
                ctx.add_source_hash(hint.to_string(), merkle_hash(&hash_pairs));
            } else {
                match safe_join(root, hint) {
                    Ok(full_path) if full_path.is_dir() => {
                        let dir_path = root.join(hint);
                        let entries = walk_dir(root, &dir_path)?;
                        let hash_pairs: Vec<_> = entries.iter()
                            .map(|e| (e.rel_path.clone(), e.hash.clone()))
                            .collect();
                        let key = format!("{}/", hint.trim_end_matches('/'));
                        ctx.add_source_hash(key, merkle_hash(&hash_pairs));
                    }
                    Ok(full_path) if full_path.is_file() => {
                        match hash_source_cached(root, hint, cache) {
                            Ok(hash) => ctx.add_source_hash(hint.to_string(), hash),
                            Err(e) => tracing::warn!("Could not hash hint file {}: {}", hint, e),
                        }
                    }
                    Ok(_) => tracing::warn!("Hint path not found: {}", hint),
                    Err(e) => tracing::warn!("Hint path rejected: {}: {}", hint, e),
                }
            }
        }
    }

    // Hash the scope — single merkle hash entry
    if let Some(scope_pattern) = scope {
        let hash = hash_source_cached(root, scope_pattern, cache)?;
        ctx.add_source_hash(scope_pattern.to_string(), hash);
    }

    Ok(ctx)
}

/// Read a single file and return its content (for AI tool calls)
pub fn read_file(root: &Path, relative_path: &str) -> Result<(String, String)> {
    let full_path = safe_join(root, relative_path)?;
    let content = std::fs::read_to_string(&full_path)?;
    let hash = sha256_hex(&content);
    Ok((content, hash))
}

/// Compute SHA-256 hash of a string, returned as hex
pub fn sha256_hex(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Re-hash a file and return the current hash
pub fn hash_file(root: &Path, relative_path: &str) -> Result<String> {
    let full_path = safe_join(root, relative_path)?;
    let content = std::fs::read_to_string(&full_path)?;
    Ok(sha256_hex(&content))
}

/// Hash a source entry for staleness checking. Automatically detects type:
/// - Glob patterns (containing *, ?, [, {) → merkle hash of matching files
/// - Paths ending in "/" → merkle hash of directory contents
/// - Otherwise → individual file hash (falls back to directory if path is a dir)
pub fn hash_source(root: &Path, source: &str) -> Result<String> {
    hash_source_cached(root, source, &mut std::collections::HashMap::new())
}

/// Like `hash_source` but uses a caller-supplied cache to avoid rehashing the same path twice.
pub fn hash_source_cached(root: &Path, source: &str, cache: &mut std::collections::HashMap<String, String>) -> Result<String> {
    if let Some(cached) = cache.get(source) {
        return Ok(cached.clone());
    }
    let hash = if is_glob_pattern(source) {
        let entries = walk_glob(root, source)?;
        let hash_pairs: Vec<_> = entries.iter()
            .map(|e| (e.rel_path.clone(), e.hash.clone()))
            .collect();
        merkle_hash(&hash_pairs)
    } else {
        let clean = source.trim_end_matches('/');
        let full = safe_join(root, clean)?;
        if full.is_dir() || source.ends_with('/') {
            let entries = walk_dir(root, &root.join(clean))?;
            let hash_pairs: Vec<_> = entries.iter()
                .map(|e| (e.rel_path.clone(), e.hash.clone()))
                .collect();
            merkle_hash(&hash_pairs)
        } else {
            hash_file(root, clean)?
        }
    };
    cache.insert(source.to_string(), hash.clone());
    Ok(hash)
}

/// Hash a specific line range in a file (1-indexed, inclusive).
/// Returns `(sha256, content_len_in_bytes)` where `content_len` is the byte length
/// of the joined lines (joined with `\n`). Used for line-range source tracking.
pub fn hash_file_lines(root: &Path, relative_path: &str, start: u32, end: u32) -> Result<(String, u64)> {
    let full_path = safe_join(root, relative_path)?;
    let content = std::fs::read_to_string(&full_path)?;
    let lines: Vec<&str> = content.lines().collect();
    let total = lines.len();
    let start_idx = (start as usize).saturating_sub(1).min(total);
    let end_idx = (end as usize).min(total);
    let block = lines[start_idx..end_idx].join("\n");
    let len = block.len() as u64;
    Ok((sha256_hex(&block), len))
}

/// Check whether a line-range source is still fresh, using content-following logic.
///
/// Returns `true` (fresh) if:
/// - The same line range still hashes to `stored_hash`, OR
/// - A window of the same `content_len` bytes (at any line boundary) hashes to `stored_hash`
///   (meaning the content moved but is otherwise unchanged).
///
/// Returns `false` (stale) if the content cannot be found anywhere in the file.
pub fn check_line_range_staleness(
    root: &Path,
    relative_path: &str,
    start: u32,
    end: u32,
    content_len: u64,
    stored_hash: &str,
) -> Result<bool> {
    let full_path = safe_join(root, relative_path)?;
    let content = std::fs::read_to_string(&full_path)?;
    let lines: Vec<&str> = content.lines().collect();
    let total = lines.len();
    let line_count = (end as usize).saturating_sub((start as usize).saturating_sub(1));

    // Fast path: re-hash the original range
    let start_idx = (start as usize).saturating_sub(1).min(total);
    let end_idx = (end as usize).min(total);
    let original_block = lines[start_idx..end_idx].join("\n");
    if sha256_hex(&original_block) == stored_hash {
        return Ok(true);
    }

    // Content changed at the original position — scan for it elsewhere.
    // Build cumulative byte lengths per line boundary so we can filter candidates
    // by byte span without hashing every window.
    if line_count == 0 || line_count > total {
        return Ok(false);
    }

    // cumulative[i] = total bytes of lines[0..i].join("\n")
    let mut cumulative: Vec<u64> = Vec::with_capacity(total + 1);
    cumulative.push(0);
    for (i, line) in lines.iter().enumerate() {
        let prev = cumulative[i];
        // Each line contributes line.len() bytes; lines are joined with "\n" (1 byte between each)
        let added = line.len() as u64 + if i > 0 { 1 } else { 0 };
        cumulative.push(prev + added);
    }

    for window_start in 0..=(total - line_count) {
        let window_end = window_start + line_count;
        // Byte span of lines[window_start..window_end].join("\n")
        // = sum of line lengths + (line_count - 1) separator bytes
        let line_bytes: u64 = lines[window_start..window_end].iter().map(|l| l.len() as u64).sum();
        let sep_bytes = (line_count as u64).saturating_sub(1);
        let span = line_bytes + sep_bytes;

        if span == content_len {
            let block = lines[window_start..window_end].join("\n");
            if sha256_hex(&block) == stored_hash {
                return Ok(true);
            }
        }
    }

    Ok(false)
}


pub fn read_file_lines(
    root: &Path,
    relative_path: &str,
    start: usize,
    end: usize,
) -> Result<(String, String)> {
    let full_path = safe_join(root, relative_path)?;
    let content = std::fs::read_to_string(&full_path)?;
    let hash = sha256_hex(&content);

    let lines: Vec<&str> = content.lines().collect();
    let total = lines.len();
    let start_idx = start.saturating_sub(1).min(total);
    let end_idx = end.min(total);

    let mut result = String::new();
    for (i, line) in lines[start_idx..end_idx].iter().enumerate() {
        let line_num = start_idx + i + 1;
        result.push_str(&format!("{:>4} | {}\n", line_num, line));
    }

    if end_idx < total {
        result.push_str(&format!("... ({} more lines)\n", total - end_idx));
    }

    Ok((result, hash))
}

/// Search for a pattern across files in the repository.
/// Returns matching lines with file paths, line numbers, and optional context.
pub fn grep(
    root: &Path,
    pattern: &str,
    glob_filter: Option<&str>,
    max_results: usize,
    context_lines: usize,
) -> Result<String> {
    let re = regex::Regex::new(pattern)
        .map_err(|e| anyhow::anyhow!("Invalid regex '{}': {}", pattern, e))?;

    let canonical_root = root.canonicalize()?;
    let mut walker = ignore::WalkBuilder::new(root);
    walker.hidden(true).git_ignore(true).follow_links(false);

    let mut output = String::new();
    let mut match_count = 0;

    let glob_pattern = glob_filter.map(|g| Pattern::new(g)).transpose()?;

    for entry in walker.build() {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        // Containment check
        if let Ok(canonical) = path.canonicalize() {
            if !canonical.starts_with(&canonical_root) {
                continue;
            }
        }

        let relative = path.strip_prefix(root).unwrap_or(path);
        let rel_str = relative.to_string_lossy().replace('\\', "/");

        if let Some(ref gp) = glob_pattern {
            if !gp.matches(&rel_str) {
                continue;
            }
        }

        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue, // skip binary/unreadable
        };

        let lines: Vec<&str> = content.lines().collect();
        let total = lines.len();

        // Collect match line indices in this file, then render with context
        let match_indices: Vec<usize> = lines
            .iter()
            .enumerate()
            .filter(|(_, line)| re.is_match(line))
            .map(|(i, _)| i)
            .collect();

        if match_indices.is_empty() {
            continue;
        }

        if context_lines == 0 {
            // No context — compact format
            for &idx in &match_indices {
                output.push_str(&format!("{}:{}: {}\n", rel_str, idx + 1, lines[idx]));
                match_count += 1;
                if match_count >= max_results {
                    output.push_str(&format!("(truncated at {} matches)\n", max_results));
                    return Ok(output);
                }
            }
        } else {
            // With context — group overlapping ranges
            let ranges = merge_context_ranges(&match_indices, context_lines, total);

            for range in ranges {
                output.push_str(&format!("── {} ──\n", rel_str));
                for idx in range.start..range.end {
                    let marker = if match_indices.contains(&idx) { ">" } else { " " };
                    output.push_str(&format!(
                        "{} {:>4} | {}\n",
                        marker,
                        idx + 1,
                        lines[idx]
                    ));
                }
                output.push('\n');

                // Count the actual matches in this range
                for &idx in &match_indices {
                    if idx >= range.start && idx < range.end {
                        match_count += 1;
                        if match_count >= max_results {
                            output.push_str(&format!(
                                "(truncated at {} matches)\n",
                                max_results
                            ));
                            return Ok(output);
                        }
                    }
                }
            }
        }
    }

    if match_count == 0 {
        output.push_str("No matches found.\n");
    } else {
        output.push_str(&format!("\n{} match(es) total.\n", match_count));
    }

    Ok(output)
}

/// Merge overlapping context ranges so adjacent matches share one block.
fn merge_context_ranges(
    match_indices: &[usize],
    context: usize,
    total_lines: usize,
) -> Vec<std::ops::Range<usize>> {
    let mut ranges: Vec<std::ops::Range<usize>> = Vec::new();

    for &idx in match_indices {
        let start = idx.saturating_sub(context);
        let end = (idx + context + 1).min(total_lines);

        if let Some(last) = ranges.last_mut() {
            if start <= last.end {
                // Overlaps or adjacent — extend
                last.end = last.end.max(end);
                continue;
            }
        }
        ranges.push(start..end);
    }

    ranges
}

/// List files matching a glob pattern in the repository
pub fn find_files(root: &Path, pattern: &str) -> Result<String> {
    let glob_pattern = Pattern::new(pattern)?;
    let canonical_root = root.canonicalize()?;
    let mut walker = ignore::WalkBuilder::new(root);
    walker.hidden(true).git_ignore(true).follow_links(false);

    let mut matches = Vec::new();
    for entry in walker.build() {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        if let Ok(canonical) = path.canonicalize() {
            if !canonical.starts_with(&canonical_root) {
                continue;
            }
        }

        let relative = path.strip_prefix(root).unwrap_or(path);
        let rel_str = relative.to_string_lossy().replace('\\', "/");

        if glob_pattern.matches(&rel_str) {
            matches.push(rel_str.to_string());
        }
    }

    matches.sort();
    let mut output = String::new();
    for m in &matches {
        output.push_str(m);
        output.push('\n');
    }
    output.push_str(&format!("\n{} file(s) matched.\n", matches.len()));
    Ok(output)
}

/// List directory contents (one level deep)
pub fn list_directory(root: &Path, relative_dir: &str) -> Result<String> {
    let dir = safe_join(root, relative_dir)?;

    if !dir.is_dir() {
        anyhow::bail!("'{}' is not a directory", relative_dir);
    }

    let mut entries: Vec<(String, bool)> = Vec::new();

    for entry in std::fs::read_dir(&dir)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();

        // Skip hidden files
        if name.starts_with('.') {
            continue;
        }

        let is_dir = entry.file_type()?.is_dir();
        entries.push((name, is_dir));
    }

    entries.sort_by(|a, b| {
        // Directories first, then files
        b.1.cmp(&a.1).then(a.0.cmp(&b.0))
    });

    let mut output = String::new();
    for (name, is_dir) in &entries {
        if *is_dir {
            output.push_str(&format!("  {}/\n", name));
        } else {
            output.push_str(&format!("  {}\n", name));
        }
    }
    output.push_str(&format!("\n{} entries.\n", entries.len()));
    Ok(output)
}

/// Safely join a relative path to the root, preventing path traversal and
/// absolute path injection. This is the single security gate for all tool I/O.
fn safe_join(root: &Path, relative: &str) -> Result<PathBuf> {
    let cleaned = relative.replace('\\', "/");

    // Reject null bytes
    if cleaned.contains('\0') {
        anyhow::bail!("Path rejected (null byte): {}", relative);
    }

    // Reject absolute paths (Unix and Windows variants)
    if cleaned.starts_with('/')
        || cleaned.starts_with('~')
        || cleaned.contains("://")
        || (cleaned.len() >= 2 && cleaned.as_bytes()[1] == b':')
    {
        anyhow::bail!("Absolute path denied: {}", relative);
    }

    // Reject paths that try to escape via .. components
    for component in cleaned.split('/') {
        if component == ".." {
            anyhow::bail!("Path traversal denied (..): {}", relative);
        }
    }

    // Reject empty path (use "." for root)
    let effective = if cleaned.is_empty() { ".".to_string() } else { cleaned };

    let full = root.join(&effective);

    // Final canonicalize check catches symlink escapes
    let canonical_root = root.canonicalize()?;
    let canonical_full = full.canonicalize()
        .map_err(|_| anyhow::anyhow!("Path not found: {}", relative))?;

    if !canonical_full.starts_with(&canonical_root) {
        anyhow::bail!("Path escapes workspace: {}", relative);
    }

    Ok(canonical_full)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn workspace() -> PathBuf {
        // Use the real cwd as a known-existing directory
        env::current_dir().unwrap()
    }

    #[test]
    fn safe_join_rejects_absolute_unix() {
        assert!(safe_join(&workspace(), "/etc/passwd").is_err());
    }

    #[test]
    fn safe_join_rejects_absolute_windows() {
        assert!(safe_join(&workspace(), "C:\\Windows\\System32").is_err());
    }

    #[test]
    fn safe_join_rejects_dot_dot() {
        assert!(safe_join(&workspace(), "../../../etc/passwd").is_err());
        assert!(safe_join(&workspace(), "src/../../secret").is_err());
    }

    #[test]
    fn safe_join_rejects_tilde() {
        assert!(safe_join(&workspace(), "~/.ssh/id_rsa").is_err());
    }

    #[test]
    fn safe_join_rejects_null_byte() {
        assert!(safe_join(&workspace(), "src\0/evil").is_err());
    }

    #[test]
    fn safe_join_rejects_uri() {
        assert!(safe_join(&workspace(), "file:///etc/passwd").is_err());
    }

    #[test]
    fn safe_join_allows_valid_relative() {
        // Cargo.toml exists at the repo root
        assert!(safe_join(&workspace(), "Cargo.toml").is_ok());
    }

    #[test]
    fn safe_join_allows_nested_relative() {
        // src/main.rs exists
        assert!(safe_join(&workspace(), "src/main.rs").is_ok());
    }

    #[test]
    fn safe_join_empty_resolves_to_root() {
        let result = safe_join(&workspace(), "").unwrap();
        assert_eq!(result, workspace().canonicalize().unwrap());
    }

    #[test]
    fn test_is_glob_pattern() {
        assert!(is_glob_pattern("src/**/*.rs"));
        assert!(is_glob_pattern("*.txt"));
        assert!(is_glob_pattern("src/??.rs"));
        assert!(is_glob_pattern("src/{a,b}.rs"));
        assert!(is_glob_pattern("src/[abc].rs"));
        assert!(!is_glob_pattern("src/main.rs"));
        assert!(!is_glob_pattern("src/copilot/"));
    }

    #[test]
    fn test_merkle_hash_deterministic() {
        let entries_a = vec![
            ("b.rs".to_string(), "hash_b".to_string()),
            ("a.rs".to_string(), "hash_a".to_string()),
        ];
        let entries_b = vec![
            ("a.rs".to_string(), "hash_a".to_string()),
            ("b.rs".to_string(), "hash_b".to_string()),
        ];
        assert_eq!(merkle_hash(&entries_a), merkle_hash(&entries_b));
    }

    #[test]
    fn test_merkle_hash_detects_changes() {
        let original = vec![
            ("a.rs".to_string(), "hash_a".to_string()),
            ("b.rs".to_string(), "hash_b".to_string()),
        ];
        let modified = vec![
            ("a.rs".to_string(), "hash_a_changed".to_string()),
            ("b.rs".to_string(), "hash_b".to_string()),
        ];
        let added = vec![
            ("a.rs".to_string(), "hash_a".to_string()),
            ("b.rs".to_string(), "hash_b".to_string()),
            ("c.rs".to_string(), "hash_c".to_string()),
        ];
        assert_ne!(merkle_hash(&original), merkle_hash(&modified));
        assert_ne!(merkle_hash(&original), merkle_hash(&added));
    }

    #[test]
    fn test_hash_source_file() {
        let root = workspace();
        let file_hash = hash_source(&root, "Cargo.toml").unwrap();
        assert_eq!(file_hash, hash_file(&root, "Cargo.toml").unwrap());
    }

    #[test]
    fn test_hash_source_directory() {
        let root = workspace();
        let dir_hash = hash_source(&root, "src").unwrap();
        let dir_hash_slash = hash_source(&root, "src/").unwrap();
        assert_eq!(dir_hash, dir_hash_slash);
        assert_ne!(dir_hash, hash_source(&root, "Cargo.toml").unwrap());
    }

    #[test]
    fn test_hash_source_glob() {
        let root = workspace();
        let glob_hash = hash_source(&root, "src/**/*.rs").unwrap();
        assert!(!glob_hash.is_empty());
        assert_ne!(glob_hash, hash_source(&root, "Cargo.toml").unwrap());
    }
}
