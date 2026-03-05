---
generated_by: faqifai
source: faqifai.faq
toc:
- question: How does the JSON-RPC transport work between faqifai and the Copilot CLI, and what framing protocol is used?
  anchor: '#how-does-the-json-rpc-transport-work-between-faqifai-and-the-copilot-cli-and-what-framing-protocol-is-used'
  generated_at: 2026-03-05T20:23:44.962120200Z
  sources:
  - path: src/**
    sha256: 6b79582c692db19a0955f3abfea791db3b27aeb1ef2cda7ff77f22d1ec163aad
  - path: src/copilot/client.rs
    sha256: a2d9086a8131e1daece002083edaaee59aca50f74a7e2832d970c94bb903f050
  - path: src/copilot/jsonrpc.rs
    sha256: 6a99b8c156c39fe4fdecddc4e840e3d0d5db573f6463b74b0cf0965fb4fe0777
- question: What security measures prevent the AI's tool calls from accessing files outside the workspace?
  anchor: '#what-security-measures-prevent-the-ais-tool-calls-from-accessing-files-outside-the-workspace'
  generated_at: 2026-03-05T20:23:44.962120200Z
  sources:
  - path: src/**
    sha256: 6b79582c692db19a0955f3abfea791db3b27aeb1ef2cda7ff77f22d1ec163aad
  - path: src/codebase.rs
    sha256: e8a9bc3f9e9fd38efe4bec8deec05679c70b00e16948e2e227427c3e24612c71
- question: How does the tool decide whether a previously generated answer is stale and needs regeneration?
  anchor: '#how-does-the-tool-decide-whether-a-previously-generated-answer-is-stale-and-needs-regeneration'
  generated_at: 2026-03-05T20:21:02.133385900Z
  sources:
  - path: src/ai.rs
    sha256: 8242d075db8efac187da80516440af1a560d95efd83861777ab2d238bda8a1d8
  - path: src/codebase.rs
    sha256: e8a9bc3f9e9fd38efe4bec8deec05679c70b00e16948e2e227427c3e24612c71
  - path: src/orchestrator.rs
    sha256: 6a60e9dfb25e871b35f8686e4ec2e30991fd7f015296875a3c91444e129e5bec
  - path: src/state.rs
    sha256: 7b8fec453bb1dfe5c1f97dc4074e77a20822db24e2b2231eb0841fd5133f808e
- question: What is the eval tool and how does it differ from the other AI tools? What are its limitations compared to Python?
  anchor: '#what-is-the-eval-tool-and-how-does-it-differ-from-the-other-ai-tools-what-are-its-limitations-compared-to-python'
  generated_at: 2026-03-05T20:23:44.962120200Z
  sources:
  - path: src/ai.rs
    sha256: 8242d075db8efac187da80516440af1a560d95efd83861777ab2d238bda8a1d8
  - path: src/codebase.rs
    sha256: e8a9bc3f9e9fd38efe4bec8deec05679c70b00e16948e2e227427c3e24612c71
  - path: src/eval.rs
    sha256: 8be069f1dbe8b14f8aff49fde611025d908bc208b0221f8f57a3a56f8bdbb4d2
- question: How are multiple questions answered concurrently, and what controls the parallelism?
  anchor: '#how-are-multiple-questions-answered-concurrently-and-what-controls-the-parallelism'
  generated_at: 2026-03-05T20:23:44.962120200Z
  sources:
  - path: src/**
    sha256: 6b79582c692db19a0955f3abfea791db3b27aeb1ef2cda7ff77f22d1ec163aad
  - path: src/ai.rs
    sha256: 8242d075db8efac187da80516440af1a560d95efd83861777ab2d238bda8a1d8
  - path: src/orchestrator.rs
    sha256: 6a60e9dfb25e871b35f8686e4ec2e30991fd7f015296875a3c91444e129e5bec
- question: What prompts are given to Copilot for generating answers?
  anchor: '#what-prompts-are-given-to-copilot-for-generating-answers'
  generated_at: 2026-03-05T20:21:02.133385900Z
  sources:
  - path: src/ai.rs
    sha256: 8242d075db8efac187da80516440af1a560d95efd83861777ab2d238bda8a1d8
- question: how does faqifai optimize token and request usage?
  anchor: '#how-does-faqifai-optimize-token-and-request-usage'
  generated_at: 2026-03-05T20:23:44.962120200Z
  sources:
  - path: src/ai.rs
    sha256: 8242d075db8efac187da80516440af1a560d95efd83861777ab2d238bda8a1d8
  - path: src/codebase.rs
    sha256: e8a9bc3f9e9fd38efe4bec8deec05679c70b00e16948e2e227427c3e24612c71
  - path: src/config.rs
    sha256: d6ec013fdc917e4131a90dcca1cf868544bfe7cb460bb15eaf7901202cf97753
  - path: src/copilot/session.rs
    sha256: afe8c6bef8fab852becda71beb75823aa0fa0851ecc112f01f9b9f2c8b76776c
  - path: src/eval.rs
    sha256: 8be069f1dbe8b14f8aff49fde611025d908bc208b0221f8f57a3a56f8bdbb4d2
  - path: src/orchestrator.rs
    sha256: 6a60e9dfb25e871b35f8686e4ec2e30991fd7f015296875a3c91444e129e5bec
  - path: src/state.rs
    sha256: 7b8fec453bb1dfe5c1f97dc4074e77a20822db24e2b2231eb0841fd5133f808e
- question: What models does faqifai use?
  anchor: '#what-models-does-faqifai-use'
  generated_at: 2026-03-05T20:23:44.962120200Z
  sources:
  - path: README.md
    sha256: 8e2bceecaeacf0ca684ff519a8a3db7590e2180fef8c4c2638b058a4229ec304
  - path: src/ai.rs
    sha256: 8242d075db8efac187da80516440af1a560d95efd83861777ab2d238bda8a1d8
  - path: src/main.rs
    sha256: f6b7df77fb8063394ff0ffb82dbbcf99f0b5bd03409e252681b0ff1e8793eb2b
  - path: src/orchestrator.rs
    sha256: 6a60e9dfb25e871b35f8686e4ec2e30991fd7f015296875a3c91444e129e5bec
---

## Contents

- [How does the JSON-RPC transport work between faqifai and the Copilot CLI, and what framing protocol is used?](#how-does-the-json-rpc-transport-work-between-faqifai-and-the-copilot-cli-and-what-framing-protocol-is-used)
- [What security measures prevent the AI's tool calls from accessing files outside the workspace?](#what-security-measures-prevent-the-ais-tool-calls-from-accessing-files-outside-the-workspace)
- [How does the tool decide whether a previously generated answer is stale and needs regeneration?](#how-does-the-tool-decide-whether-a-previously-generated-answer-is-stale-and-needs-regeneration)
- [What is the eval tool and how does it differ from the other AI tools? What are its limitations compared to Python?](#what-is-the-eval-tool-and-how-does-it-differ-from-the-other-ai-tools-what-are-its-limitations-compared-to-python)
- [How are multiple questions answered concurrently, and what controls the parallelism?](#how-are-multiple-questions-answered-concurrently-and-what-controls-the-parallelism)
- [What prompts are given to Copilot for generating answers?](#what-prompts-are-given-to-copilot-for-generating-answers)
- [how does faqifai optimize token and request usage?](#how-does-faqifai-optimize-token-and-request-usage)
- [What models does faqifai use?](#what-models-does-faqifai-use)

---

# How does the JSON-RPC transport work between faqifai and the Copilot CLI, and what framing protocol is used?

Perfect! Now I have a complete understanding of the transport. Let me compile the answer:

## Transport Architecture

faqifai communicates with the Copilot CLI using **JSON-RPC 2.0 over standard input/output (stdio)** with **Content-Length framing**, as implemented in [jsonrpc.rs:1](./src/copilot/jsonrpc.rs#L1).

## Process Setup

The Copilot CLI is spawned as a child process with `--headless --stdio` flags ([client.rs:34](./src/copilot/client.rs#L34)), connecting stdin/stdout pipes:

```rust
Command::new(&cli_path)
    .args(["--headless", "--no-auto-update", "--log-level", "info", "--stdio"])
    .stdin(std::process::Stdio::piped())
    .stdout(std::process::Stdio::piped())
```

The `Transport` is initialized with these pipe handles ([jsonrpc.rs:65](./src/copilot/jsonrpc.rs#L65)), spawning a dedicated read loop task to handle incoming messages.

## Framing Protocol: Content-Length Headers

Messages use **HTTP-style Content-Length framing** — identical to the Language Server Protocol (LSP):

### Writing Messages ([jsonrpc.rs:153-163](./src/copilot/jsonrpc.rs#L153-L163))

1. Serialize the JSON-RPC message to bytes
2. Write header: `Content-Length: <byte_count>\r\n\r\n`
3. Write the JSON body
4. Flush

```rust
let data = serde_json::to_vec(message)?;
let header = format!("Content-Length: {}\r\n\r\n", data.len());

stdin.write_all(header.as_bytes()).await?;
stdin.write_all(&data).await?;
stdin.flush().await?;
```

### Reading Messages ([jsonrpc.rs:191-230](./src/copilot/jsonrpc.rs#L191-L230))

The read loop continuously:

1. **Reads header lines** until encountering a blank line (`\r\n`)
2. **Extracts the `Content-Length` value** from the header ([jsonrpc.rs:217-218](./src/copilot/jsonrpc.rs#L217-L218))
3. **Reads exactly that many bytes** as the message body ([jsonrpc.rs:227-228](./src/copilot/jsonrpc.rs#L227-L228))
4. **Deserializes the JSON** and dispatches it

```rust
// Parse Content-Length from headers
if let Some(len_str) = trimmed.strip_prefix("Content-Length: ") {
    content_length = len_str.parse()?;
}

// Read exact byte count
let mut body = vec![0u8; content_length];
reader.read_exact(&mut body).await?;
let raw: Value = serde_json::from_slice(&body)?;
```

## Message Flow

The transport is **bidirectional** and **asynchronous**:

- **Client→Server**: faqifai sends JSON-RPC requests (e.g., `session.create`, `session.send`) via `Transport::request()` ([jsonrpc.rs:105-136](./src/copilot/jsonrpc.rs#L105-L136)), which generates a UUID message ID and awaits the response via a oneshot channel
- **Server→Client**: The Copilot CLI sends:
  - **Responses** to client requests, matched by message ID ([jsonrpc.rs:327-335](./src/copilot/jsonrpc.rs#L327-L335))
  - **Requests** for tool calls or permissions, handled by registered callbacks ([jsonrpc.rs:241-318](./src/copilot/jsonrpc.rs#L241-L318))
  - **Notifications** like `session.event`, dispatched to subscribers ([jsonrpc.rs:320-325](./src/copilot/jsonrpc.rs#L320-L325))

## Protocol Implementation Notes

- **Line endings**: Headers use `\r\n` ([jsonrpc.rs:155](./src/copilot/jsonrpc.rs#L155))
- **Blank line separator**: A single `\r\n` after all headers signals the body starts ([jsonrpc.rs:213-214](./src/copilot/jsonrpc.rs#L213-L214))
- **Exact byte reads**: The body is read as raw bytes, not line-by-line, using `read_exact()` ([jsonrpc.rs:228](./src/copilot/jsonrpc.rs#L228))
- **Graceful EOF**: When stdout closes (0 bytes read), the loop terminates normally ([jsonrpc.rs:208-209](./src/copilot/jsonrpc.rs#L208-L209))
- **Bidirectional writes**: Server→client responses are sent through the same stdin handle during the read loop ([jsonrpc.rs:266-270](./src/copilot/jsonrpc.rs#L266-L270), [jsonrpc.rs:293-297](./src/copilot/jsonrpc.rs#L293-L297))

This protocol matches the [official Copilot SDK](https://github.com/github/copilot-sdk) Go implementation ([mod.rs:3-4](./src/copilot/mod.rs#L3-L4)).

# What security measures prevent the AI's tool calls from accessing files outside the workspace?

Perfect! Now I have a complete understanding of all security measures. Let me compose the answer:

The security model consists of **four defense layers** that together prevent the AI from accessing files outside the workspace:

## 1. Tool Exclusion

The [`excluded_tools()`](./src/ai.rs#L276-L285) function blocks write-capable tools provided by the GitHub Copilot SDK:

```rust
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
```

This list is passed to the Copilot session configuration at [`ai.rs:327`](./src/ai.rs#L327), ensuring these tools are never available to the AI. The AI only has access to read-only built-in tools plus two custom tools: `analyze` (for Starlark script execution) and `mark_unchanged` (a flag for incremental updates).

## 2. Working Directory Restriction

The Copilot CLI process is spawned with its working directory set to the repository root:

- In [`client.rs:40-42`](./src/copilot/client.rs#L40-L42), the client sets `cmd.current_dir(cwd)` when launching copilot
- In [`ai.rs:328`](./src/ai.rs#L328) and [`ai.rs:449`](./src/ai.rs#L449), the workspace root is passed as `working_directory`

This constrains the copilot process's execution context to the workspace.

## 3. Path Validation Gate: `safe_join()`

**Every file access operation** funnels through the [`safe_join()`](./src/codebase.rs#L496-L535) function, which implements multiple checks:

```rust
fn safe_join(root: &Path, relative: &str) -> Result<PathBuf>
```

### Rejection rules (lines 499-518):

1. **Null bytes**: Blocks paths containing `\0`
2. **Absolute paths**: Rejects paths starting with:
   - `/` (Unix absolute)
   - `~` (home directory expansion)
   - Containing `://` (URI schemes like `file://`)
   - Drive letters (Windows `C:`, etc.) — checks if byte at position 1 is `:`
3. **Path traversal**: Rejects any path containing `..` components
4. **Empty paths**: Normalizes empty string to `.` (current directory)

### Canonicalization check (lines 525-532):

After basic validation, `safe_join` performs a **definitive security check**:

```rust
let canonical_root = root.canonicalize()?;
let canonical_full = full.canonicalize()
    .map_err(|_| anyhow::anyhow!("Path not found: {}", relative))?;

if !canonical_full.starts_with(&canonical_root) {
    anyhow::bail!("Path escapes workspace: {}", relative);
}
```

This resolves both paths to their absolute, symlink-free forms and verifies that the requested file's canonical path is a descendant of the workspace root. This **catches symlink-based escapes** that might otherwise bypass string-based checks.

## 4. Defense in Depth: Per-Operation Containment Checks

Even though `safe_join` is the primary gate, file traversal operations (`walk_glob`, `walk_dir`, `grep`, `find_files`) perform **redundant containment checks** on every discovered file:

**Example from [`grep` at lines 301-305](./src/codebase.rs#L301-L305):**

```rust
if let Ok(canonical) = path.canonicalize() {
    if !canonical.starts_with(&canonical_root) {
        continue;
    }
}
```

This pattern appears in:
- [`walk_glob:77-78`](./src/codebase.rs#L77-L78)
- [`walk_dir:112-113`](./src/codebase.rs#L112-L113)  
- [`grep:301-304`](./src/codebase.rs#L301-L304)
- [`find_files:430-432`](./src/codebase.rs#L430-L432)

These redundant checks ensure that even if a file scanner encounters paths through other means (e.g., following symlinks discovered during directory traversal), they are filtered out before being processed.

## Tool Function Mapping

All file I/O tools exposed to the AI ultimately call `codebase` functions that use `safe_join`:

| Tool/Function | Entry Point | Security Gate |
|--------------|-------------|---------------|
| `read_file()` | [`codebase::read_file`](./src/codebase.rs#L185) → [`safe_join`](./src/codebase.rs#L186) | Line 186 |
| `read_file_lines()` | [`codebase::read_file_lines`](./src/codebase.rs#L243) → [`safe_join`](./src/codebase.rs#L249) | Line 249 |
| `list_dir()` | [`codebase::list_directory`](./src/codebase.rs#L455) → [`safe_join`](./src/codebase.rs#L456) | Line 456 |
| `grep()` | [`codebase::grep`](./src/codebase.rs#L273) | Uses walker + containment checks |
| `find_files()` | [`codebase::find_files`](./src/codebase.rs#L415) | Uses walker + containment checks |

The Starlark `analyze` tool exposes these same functions via [`eval.rs`](./src/eval.rs#L62-L122), ensuring that even script-based access is constrained.

## Test Coverage

The [`codebase::tests` module](./src/codebase.rs#L537-L606) validates rejection of:
- Unix absolute paths (`/etc/passwd`)
- Windows absolute paths (`C:\Windows\System32`)
- Path traversal (`../../etc/passwd`, `src/../../secret`)
- Tilde expansion (`~/.ssh/id_rsa`)
- Null bytes (`src\0/evil`)
- URI schemes (`file:///etc/passwd`)

And confirms that valid relative paths are accepted ([`safe_join_allows_valid_relative`](./src/codebase.rs#L579-L582), [`safe_join_allows_nested_relative`](./src/codebase.rs#L584-L588)).

# How does the tool decide whether a previously generated answer is stale and needs regeneration?

## Detection triggers

An answer is considered stale and needs regeneration when any of three conditions are met:

1. **Never generated** — The output file doesn't exist or doesn't contain an entry for the question
2. **TTL expired** — A time-to-live is configured and has elapsed since generation
3. **Sources changed** — One or more source files tracked in the answer's metadata have changed content

## How it works

### 1. Source tracking during generation

When the AI generates an answer, it is instructed to append a special comment block listing all files it actually read ([ai.rs:48-57](./src/ai.rs#L48-L57)):

```html
<!-- faqifai-sources
src/path/to/file.rs
src/path/to/other.rs
-->
```

The orchestrator extracts these paths from the answer ([ai.rs:139-155](./src/ai.rs#L139-L155)), computes their SHA-256 hashes, and stores them in the output file's frontmatter as a list of `SourceFile` entries ([state.rs:27-32](./src/state.rs#L27-L32)):

```rust
pub struct SourceFile {
    pub path: String,
    pub sha256: String,
}
```

### 2. Staleness checking

When running `faqifai run` or `faqifai status`, the tool calls `state::check_staleness()` for each question ([state.rs:100-152](./src/state.rs#L100-L152)):

**Step 1: Load frontmatter**  
Parse the output markdown file to extract the YAML frontmatter containing the TOC ([state.rs:48-62](./src/state.rs#L48-L62)). If the file or question entry doesn't exist, return `Staleness::NeverGenerated`.

**Step 2: Check TTL (if configured)**  
If a TTL like `"7d"` or `"24h"` is set, compare the question's `generated_at` timestamp against the current time ([state.rs:122-129](./src/state.rs#L122-L129)). If the age exceeds the TTL duration, return `Staleness::TtlExpired`.

**Step 3: Verify source hashes**  
For each `SourceFile` in the TOC entry, recompute the current hash and compare ([state.rs:132-149](./src/state.rs#L132-L149)):

- **Files**: Use `codebase::hash_source_cached()` which computes SHA-256 of file content
- **Directories** (paths ending in `/`): Compute a merkle tree hash of all contained files ([codebase.rs:40-54](./src/codebase.rs#L40-L54))
- **Glob patterns** (containing `*`, `?`, `[`, `{`): Walk matching files and compute a merkle tree hash ([codebase.rs:36-38](./src/codebase.rs#L36-L38), [codebase.rs:63-96](./src/codebase.rs#L63-L96))

The merkle hash aggregates `(path, hash)` pairs in sorted order, so it detects:
- Content changes in existing files
- New files added to a directory/pattern
- Files deleted from a directory/pattern

If any hash mismatch is found, the changed paths are collected and returned as `Staleness::SourcesChanged(Vec<String>)`.

**Step 4: Return status**  
If all checks pass, return `Staleness::Fresh` ([state.rs:151](./src/state.rs#L151)).

### 3. Regeneration behavior

In [orchestrator.rs:74-108](./src/orchestrator.rs#L74-L108), only questions with non-fresh staleness are sent to the AI. The orchestrator passes:

- The previous answer for context ([orchestrator.rs:138](./src/orchestrator.rs#L138))
- A list of changed source paths so the AI knows where to focus ([orchestrator.rs:118-121](./src/orchestrator.rs#L118-L121))

The AI can call the `mark_unchanged` tool if after verification it determines the previous answer is still accurate ([ai.rs:264-271](./src/ai.rs#L264-L271), [ai.rs:345-356](./src/ai.rs#L345-L356)). When this tool is called, it sets an atomic flag ([ai.rs:406-414](./src/ai.rs#L406-L414)), and the orchestrator reuses the previous answer instead of the (potentially empty) response. Otherwise, the AI generates a new answer with an updated sources block.

### 4. Hash caching

To avoid redundant I/O when checking multiple questions that reference the same files, the orchestrator maintains a `HashMap<String, String>` cache that's shared across all staleness checks in a single run ([orchestrator.rs:24](./src/orchestrator.rs#L24), [state.rs:105](./src/state.rs#L105), [codebase.rs:215-240](./src/codebase.rs#L215-L240)).

## Important details

- **Fallback sources**: If the AI doesn't emit a sources block, the system falls back to the scope/hints specified in the `.faq` file for staleness tracking ([ai.rs:360-377](./src/ai.rs#L360-L377))
- **Directory notation**: Listing `src/some/dir/` (with trailing slash) tracks all files under that directory. Adding/removing files triggers regeneration ([codebase.rs:226-233](./src/codebase.rs#L226-L233))
- **Glob support**: Patterns like `src/**/*.rs` are detected by checking for glob metacharacters and tracked using a merkle hash of all matching files ([codebase.rs:36-38](./src/codebase.rs#L36-L38), [codebase.rs:219-224](./src/codebase.rs#L219-L224))
- **Error handling**: If a tracked file is deleted or becomes unreadable, it's treated as a change and triggers regeneration ([state.rs:140-143](./src/state.rs#L140-L143))

# What is the eval tool and how does it differ from the other AI tools? What are its limitations compared to Python?

Perfect! Now I have enough information to write a comprehensive answer. Let me create the answer:

The **analyze tool** (also called "eval" in the source code implementation) is a custom Starlark script execution environment that allows the AI to perform multi-step codebase analysis operations in a single tool call. It's distinct from GitHub Copilot's built-in tools in both purpose and implementation.

## Purpose and Design

The analyze tool exists to reduce API roundtrips by batching multiple file operations into one script execution. Instead of making sequential tool calls like:
1. Read file A
2. Parse it
3. Search for references  
4. Read file B
5. Cross-reference data

...the AI can write a single Starlark script that performs all these steps and returns the aggregated result.

## Implementation

The tool is implemented in [eval.rs](./src/eval.rs) using the [Starlark](https://github.com/bazelbuild/starlark) interpreter (a Python-like configuration language created by Google). Key characteristics:

- **Sandboxed execution**: Scripts run in an isolated environment with no network access, no shell execution, and no write capabilities ([eval.rs:15-27](./src/eval.rs#L15-L27))
- **Path scoping**: All file operations are restricted to the workspace root via the `EvalContext` ([eval.rs:18-20](./src/eval.rs#L18-L20))
- **Native function bindings**: Starlark builtins are exposed via the `codebase_builtins` module ([eval.rs:61-168](./src/eval.rs#L61-L168))
- **String result**: Scripts return their final expression as a string ([eval.rs:48-51](./src/eval.rs#L48-L51))

## Built-in Functions

The analyze tool provides these Starlark built-ins ([eval.rs:62-168](./src/eval.rs#L62-L168)):

**File I/O:**
- `read_file(path)` — Read entire file contents
- `read_file_lines(path, start, end)` — Read line range with line numbers
- `find_files(pattern)` — Glob file search
- `list_dir(path)` — Directory listing  
- `grep(pattern, glob="", max_results=50)` — Regex search across files

**Text processing:**
- `lines(text)` — Split into list of lines
- `regex_find(pattern, text)` — Find all matches
- `regex_match(pattern, text)` — Test if pattern matches

**Data parsing:**
- `json_parse(text)` — Parse JSON into Starlark dict/list
- `toml_parse(text)` — Parse TOML into Starlark dict

## Differences from Other AI Tools

### vs. Built-in Copilot Tools

GitHub Copilot CLI provides built-in tools that map directly to the codebase functions in [codebase.rs](./src/codebase.rs):

- `view` / `read_file` — Direct file reading ([codebase.rs:185-190](./src/codebase.rs#L185-L190))
- `grep` — Regex search ([codebase.rs:273-414](./src/codebase.rs#L273-L414))
- `glob` / `find_files` — File pattern matching ([codebase.rs:415-453](./src/codebase.rs#L415-L453))
- `list_dir` — Directory listing ([codebase.rs:455-475](./src/codebase.rs#L455-L475))

The key differences:

| Feature | Built-in Tools | Analyze Tool |
|---------|---------------|--------------|
| Execution | Single file/search operation | Multi-step script with control flow |
| API calls | One call per operation | One call for entire script |
| Language | N/A (direct tool invocation) | Starlark with full scripting |
| Data manipulation | None (raw output) | Parse, filter, aggregate, format |
| Use case | Simple reads/searches | Complex cross-referencing, parsing configs |

### vs. Write Tools

faqifai **excludes** Copilot's write tools ([ai.rs:275-285](./src/ai.rs#L275-L285)):
- `edit_file`
- `create_file`  
- `delete_file`

The analyze tool is strictly read-only — it cannot modify files ([eval.rs:62-122](./src/eval.rs#L62-L122)).

## Limitations Compared to Python

Starlark is intentionally restricted compared to Python:

### Language Features NOT Available
From [ai.rs:82](./src/ai.rs#L82) and verified in the Starlark dialect configuration ([eval.rs:37-41](./src/eval.rs#L37-L41)):

- **No modules**: `import`, `from`, `__import__`
- **No classes**: `class` keyword doesn't exist
- **No exceptions**: `try`, `except`, `finally`, `raise` 
- **No while loops**: Must use `for x in range(n)` instead
- **No sets**: Only lists and dicts
- **No generators**: `yield`, generator expressions
- **No context managers**: `with` statement
- **No variable scoping**: `global`, `nonlocal`

### Restricted Features

**f-strings** ([eval.rs:39](./src/eval.rs#L39)): Only simple variable interpolation works:
```starlark
# Works
f"value is {x}"

# Does NOT work (use concatenation instead)
f"value is {d['key']}"  # Must be: "value is " + str(d["key"])
```

Verified in tests at [eval.rs:315-321](./src/eval.rs#L315-L321).

**Error handling**: Errors propagate immediately — no catching them ([ai.rs:84](./src/ai.rs#L84)). Scripts must check values before using them.

**Mutability**: Strings are immutable, but lists and dicts are mutable (standard Starlark behavior).

### What IS Available

**Control flow**: `if`/`elif`/`else`, `for`, `def`, `lambda`, `return`

**Comprehensions**: List and dict comprehensions work ([eval.rs:342-348](./src/eval.rs#L342-L348))

**Standard builtins**: `len`, `range`, `enumerate`, `zip`, `sorted`, `reversed`, `str`, `int`, `float`, `bool`, `list`, `dict`, `type`, `hasattr`, `getattr`, `repr`, `min`, `max`, `any`, `all`

**String/list/dict methods**: All standard methods are available ([ai.rs:88-90](./src/ai.rs#L88-L90))

**Tuple unpacking**: Works in for loops — `for k, v in d.items()` is valid ([eval.rs:370-380](./src/eval.rs#L370-L380))

**format() method**: `.format()` is supported as an alternative to limited f-strings ([eval.rs:382-390](./src/eval.rs#L382-L390))

## Registration in AI Sessions

The analyze tool is registered as a custom tool handler in [ai.rs:242-262](./src/ai.rs#L242-L262) with this JSON schema:

```json
{
  "name": "analyze",
  "description": "Run a Starlark script for multi-step codebase analysis...",
  "parameters": {
    "type": "object",
    "properties": {
      "intent": "One-line description...",
      "script": "Starlark script. The last expression is the return value."
    },
    "required": ["intent", "script"]
  }
}
```

The handler invokes `eval::evaluate()` ([ai.rs:386-403](./src/ai.rs#L386-L403)) and returns success/failure as a `ToolResult`.

## When to Use

The system prompt instructs the AI to use `analyze` instead of making 3+ sequential read/grep calls ([ai.rs:38](./src/ai.rs#L38)). Example scenarios from the system prompt ([ai.rs:94-127](./src/ai.rs#L94-L127)):

- Cross-referencing function definitions with their call sites
- Parsing Cargo.toml/package.json and extracting dependency info
- Reading multiple files and aggregating data
- Pattern matching across files and counting results

# How are multiple questions answered concurrently, and what controls the parallelism?



# What prompts are given to Copilot for generating answers?

Now I have all the information needed. Let me create a comprehensive answer about the prompts given to Copilot.

## System Prompt

The core system prompt is defined in [`ai.rs:16-127`](./src/ai.rs#L16-L127) as the `SYSTEM_PROMPT` constant. It instructs the AI to act as a "codebase research agent generating reference documentation."

### Key directives:

1. **Be thorough** — Trace call chains, follow imports, read config files, include edge cases
2. **Be factual and unopinionated** — State what code does, not what it should do; never speculate
3. **Reference source files** — Use markdown links with relative paths; link text must include filename
4. **Use tools aggressively** — grep, glob, view files; use `analyze` tool for multi-step analysis
5. **Structure for scanability** — Use headings, bullets, code blocks; lead with direct answer
6. **Output only answer body** — No preamble, meta-commentary, or sign-off
7. **Handle previous answers** — When regenerating, verify claims and call `mark_unchanged` if nothing changed
8. **Append sources block** — End with `<!-- faqifai-sources ... -->` listing files read

The prompt also includes a complete **Starlark scripting reference** for the `analyze` tool, documenting built-in functions, language limitations, and examples.

### Path resolution

The system prompt contains a `{output_dir}` placeholder that gets replaced at runtime ([`ai.rs:171-174`](./src/ai.rs#L171-L174)) with the relative prefix needed to link from the output file back to the repo root (e.g., `./` for root-level files, `../` for `docs/file.md`, `../../` for `docs/api/file.md`).

## User Message

The user message is constructed by `build_user_message` ([`ai.rs:177-239`](./src/ai.rs#L177-L239)) and can include:

### 1. Context (optional)
```markdown
## Context

{context text from .faq file}
```

### 2. Suggested Starting Points (optional)
```markdown
## Suggested Starting Points

- `path/to/file.rs`
- `other/file.rs`
```

Listed from the `.faq` file's `hint` field.

### 3. Previous Answer (for regeneration)

When regenerating a stale answer ([`ai.rs:204-233`](./src/ai.rs#L204-L233)):

```markdown
## Previous Answer (for reference only)

The following is a previous answer to this question. It may be partially or fully outdated...

**Detected changes since last generation:**
- `src/file.rs`
- `src/other/`

Pay special attention to these changed sources...

**Files previously tracked as relevant** (update this list in your sources block...):
- `src/file.rs`
- `src/other.rs`

<previous_answer>
{the old answer text}
</previous_answer>
```

### 4. Question
```markdown
## Question

{question text from .faq file}
```

## Tool Definitions

Two custom tools are defined ([`ai.rs:242-273`](./src/ai.rs#L242-L273)):

### `analyze`
Runs a Starlark script for multi-step codebase analysis (cross-referencing files, parsing TOML/JSON, etc.). Requires `intent` (description) and `script` parameters.

**Handler:** [`ai.rs:396-404`](./src/ai.rs#L396-L404) — executes the script via the `eval` module and returns results or errors.

### `mark_unchanged`
Signals that the previous answer is still accurate after verification. Takes no arguments.

**Handler:** [`ai.rs:406-414`](./src/ai.rs#L406-L414) — sets an atomic flag ([`ai.rs:316`](./src/ai.rs#L316)) that causes the system to reuse the previous answer instead of the AI's output ([`ai.rs:345-356`](./src/ai.rs#L345-L356)).

## Excluded Tools

Three built-in Copilot CLI tools are explicitly excluded ([`ai.rs:276-285`](./src/ai.rs#L276-L285)):
- `edit_file`
- `create_file` 
- `delete_file`

This ensures the AI has read-only access to the codebase.

## Session Configuration

The session is created with ([`ai.rs:320-331`](./src/ai.rs#L320-L331)):
- **Model** — Specified by CLI argument (e.g., `claude-sonnet-4`)
- **System message** — The processed system prompt with `{output_dir}` replaced
- **Tools** — The two custom tool definitions
- **Excluded tools** — The three write-operation tools
- **Working directory** — The repository root

# how does faqifai optimize token and request usage?

Excellent! All the key claims from the previous answer check out. The changes in session.rs are minor refinements to tool call logging but don't fundamentally change the optimization strategies. Let me now compile the comprehensive answer based on verified facts:

## Caching and Staleness Detection

faqifai avoids regenerating answers by tracking source file hashes in frontmatter ([state.rs:27-32](./src/state.rs#L27-L32)). Each TOC entry stores the files that influenced that answer along with their SHA-256 hashes ([state.rs:19-25](./src/state.rs#L19-L25)). On every run, [state.rs:100-152](./src/state.rs#L100-L152) recomputes hashes and only regenerates when:
- The answer doesn't exist yet ([state.rs:111-112](./src/state.rs#L111-L112))
- The TTL has expired ([state.rs:122-129](./src/state.rs#L122-L129))
- Any tracked source file has changed ([state.rs:132-149](./src/state.rs#L132-L149))

## Hash Cache for Deduplication

A shared `HashMap<String, String>` cache ([orchestrator.rs:24](./src/orchestrator.rs#L24)) prevents re-hashing the same file multiple times when it's referenced by multiple questions. Passed to `collect_context` ([orchestrator.rs:110-115](./src/orchestrator.rs#L110-L115)) and `check_staleness` ([orchestrator.rs:77](./src/orchestrator.rs#L77)), it ensures every file/glob is hashed at most once per run ([codebase.rs:215-219](./src/codebase.rs#L215-L219)).

## Merkle Tree Hashing for Directories and Globs

Directories and glob patterns produce a single merkle hash rather than tracking every file individually ([codebase.rs:40-54](./src/codebase.rs#L40-L54)). The merkle hash is computed from sorted (path, content_hash) pairs ([codebase.rs:43-47](./src/codebase.rs#L43-L47)), detecting additions, deletions, and content changes ([codebase.rs:42](./src/codebase.rs#L42)). This keeps frontmatter compact even when a scope includes hundreds of files.

## mark_unchanged Sentinel Tool

When the AI determines a previous answer is still accurate, it calls the `mark_unchanged` tool ([ai.rs:264-271](./src/ai.rs#L264-L271)) rather than regenerating. This sets an atomic flag ([ai.rs:316](./src/ai.rs#L316), [ai.rs:410](./src/ai.rs#L410)) that signals reuse of the previous answer ([ai.rs:345-352](./src/ai.rs#L345-L352)), avoiding redundant generation and preserving staleness timestamps.

## Session-per-Question Architecture

Each question gets a fresh Copilot session ([ai.rs:319-331](./src/ai.rs#L319-L331)) that's destroyed immediately after completion ([ai.rs:340-342](./src/ai.rs#L340-L342)). This prevents conversation history bloat — sessions never accumulate tool call history or context from other questions. System prompts are configured once per session ([ai.rs:322-325](./src/ai.rs#L322-L325)) rather than prepended to every message.

## Shared Copilot Process Across Workers

Concurrent workers share one `Arc<Client>` ([ai.rs:448-451](./src/ai.rs#L448-L451)), meaning one Copilot process handles all sessions ([ai.rs:441-451](./src/ai.rs#L441-L451)). Workers pull questions from a shared queue ([ai.rs:456-481](./src/ai.rs#L456-L481)), create lightweight sessions, and return results in original order ([ai.rs:503-510](./src/ai.rs#L503-L510)).

## No File Content in Prompts

The system never sends file content to the AI directly. Instead, it tracks file paths and their hashes for staleness detection ([codebase.rs:131-182](./src/codebase.rs#L131-L182)), but the AI reads files itself using the `view` tool and the `analyze` Starlark scripting tool. The user message contains only context, question text, hints (as file paths), and optionally the previous answer ([ai.rs:177-238](./src/ai.rs#L177-L238)).

## AI-Reported Source Tracking

After the AI finishes, the system extracts sources from the answer's `<!-- faqifai-sources ... -->` block ([ai.rs:139-155](./src/ai.rs#L139-L155)). If the AI reported sources, those are hashed and stored in frontmatter ([ai.rs:360-377](./src/ai.rs#L360-L377)). If not, the system falls back to pre-computed scope/hint hashes ([ai.rs:371-373](./src/ai.rs#L371-L373)). This ensures staleness tracking reflects *what the AI actually read* rather than a broad initial scope.

## Excluded Write Tools

The Copilot session excludes `edit_file`, `create_file`, and `delete_file` ([ai.rs:276-285](./src/ai.rs#L276-L285)). This prevents the AI from modifying source code, keeps answers deterministic (they won't vary based on prior mutations), and avoids polluting the repository with generated code.

## Incremental Regeneration

When sources change, only stale questions are regenerated ([orchestrator.rs:91-143](./src/orchestrator.rs#L91-L143)). The orchestrator merges fresh answers from the existing output file with newly regenerated answers ([orchestrator.rs:198-217](./src/orchestrator.rs#L198-L217)), preserving timestamps and reducing redundant API calls.

## Starlark analyze Tool for Batch Operations

The AI can run Starlark scripts via the `analyze` tool ([ai.rs:244-262](./src/ai.rs#L244-L262)) to batch multiple file reads, cross-reference data, and parse structured files (TOML/JSON) in a single API call ([eval.rs:61-168](./src/eval.rs#L61-L168)). The system prompt instructs the AI to use this instead of chaining 3+ sequential read/grep calls ([ai.rs:38](./src/ai.rs#L38)). This reduces roundtrips — a script that reads 10 files counts as one tool call, not 10.

## TTL-Based Expiration

Questions can specify a time-to-live (e.g., `ttl = "7d"`) ([config.rs:8-9](./src/config.rs#L8-L9)). Answers older than their TTL are regenerated even if sources haven't changed ([state.rs:122-129](./src/state.rs#L122-L129)). This balances freshness with token cost — fast-moving questions get shorter TTLs, stable ones get longer TTLs or none at all.

## Parallel Hashing with Rayon

When walking directories or globs to compute merkle hashes, file content hashing happens in parallel via `rayon::par_iter` ([codebase.rs:87-93](./src/codebase.rs#L87-L93), [codebase.rs:120-126](./src/codebase.rs#L120-L126)). This speeds up staleness checks for large scopes (e.g., `src/**/*.rs` with hundreds of files).

## Compact Logging for Tool Calls

Tool calls are logged with summarized arguments ([session.rs:43-50](./src/session.rs#L43-L50)). File paths are relativized to the workspace root; out-of-root paths (e.g., `/tmp/`) are suppressed ([session.rs:427-432](./src/session.rs#L427-L432)). Long commands are truncated to 80 characters ([session.rs:461](./src/session.rs#L461)). The `mark_unchanged` tool is not logged ([session.rs:42](./src/session.rs#L42)).

## Previous Answer as Context for Regeneration

When sources change, the system provides the old answer to the AI ([ai.rs:204-233](./src/ai.rs#L204-L233)) along with a list of which files changed ([ai.rs:209-217](./src/ai.rs#L209-L217)). The AI can reuse unchanged sections, correct outdated claims, and preserve structure — often much faster than answering from scratch. The system prompt explicitly instructs the AI to verify claims and use `mark_unchanged` if nothing has changed ([ai.rs:46](./src/ai.rs#L46)).

# What models does faqifai use?

faqifai uses **`claude-sonnet-4.6`** as the default model. This is configurable via the `--model` CLI argument when running `faqifai run`.

## Configuration

- **Default model:** `claude-sonnet-4.6` — defined in [main.rs:44](./src/main.rs#L44)
- **Override:** The `--model MODEL` flag allows specifying a different model at runtime

## Implementation details

The model parameter is:

1. **Parsed from CLI** — Captured in the `Run` command struct at [main.rs:44-45](./src/main.rs#L44-L45) with `default_value = "claude-sonnet-4.6"`
2. **Passed to orchestrator** — Flows through `orchestrator::run()` at [orchestrator.rs:14](./src/orchestrator.rs#L14) 
3. **Used in AI sessions** — Applied when creating Copilot sessions via `SessionConfig` at [ai.rs:321](./src/ai.rs#L321), where the model string is set in the session configuration sent to the GitHub Copilot API

The model string is passed directly to the GitHub Copilot SDK, which supports any model available through Copilot's API. The specific model identifier format (e.g., `claude-sonnet-4.6`) follows GitHub Copilot's model naming conventions.