---
generated_by: faqifai
source: faqifai.faq
toc:
- question: How does the JSON-RPC transport work between faqifai and the Copilot CLI, and what framing protocol is used?
  anchor: '#how-does-the-json-rpc-transport-work-between-faqifai-and-the-copilot-cli-and-what-framing-protocol-is-used'
  generated_at: 2026-03-05T19:22:42.615717200Z
  sources:
  - path: src/copilot/client.rs
    sha256: a2d9086a8131e1daece002083edaaee59aca50f74a7e2832d970c94bb903f050
  - path: src/copilot/jsonrpc.rs
    sha256: 6a99b8c156c39fe4fdecddc4e840e3d0d5db573f6463b74b0cf0965fb4fe0777
  - path: src/copilot/mod.rs
    sha256: 1bb17abf43d0341ea37dfaacbe8687c1878e98e8d199c1169d7c2df38c550894
  - path: src/copilot/session.rs
    sha256: 31838cca9b3d2c3331f0a6ab0721f7ebf18b39e267d8948e34c57ff0b3b2ec61
- question: What security measures prevent the AI's tool calls from accessing files outside the workspace?
  anchor: '#what-security-measures-prevent-the-ais-tool-calls-from-accessing-files-outside-the-workspace'
  generated_at: 2026-03-05T19:22:42.615717200Z
  sources:
  - path: src/ai.rs
    sha256: bf64a29c52c3c2b49f1be78f700441e9fb38be91ff0446eda369ff157e93c88b
  - path: src/codebase.rs
    sha256: e8a9bc3f9e9fd38efe4bec8deec05679c70b00e16948e2e227427c3e24612c71
  - path: src/copilot/client.rs
    sha256: a2d9086a8131e1daece002083edaaee59aca50f74a7e2832d970c94bb903f050
  - path: src/eval.rs
    sha256: 8be069f1dbe8b14f8aff49fde611025d908bc208b0221f8f57a3a56f8bdbb4d2
- question: How does the tool decide whether a previously generated answer is stale and needs regeneration?
  anchor: '#how-does-the-tool-decide-whether-a-previously-generated-answer-is-stale-and-needs-regeneration'
  generated_at: 2026-03-05T19:22:42.615717200Z
  sources:
  - path: src/ai.rs
    sha256: bf64a29c52c3c2b49f1be78f700441e9fb38be91ff0446eda369ff157e93c88b
  - path: src/codebase.rs
    sha256: e8a9bc3f9e9fd38efe4bec8deec05679c70b00e16948e2e227427c3e24612c71
  - path: src/config.rs
    sha256: d6ec013fdc917e4131a90dcca1cf868544bfe7cb460bb15eaf7901202cf97753
  - path: src/orchestrator.rs
    sha256: b8867fbb95e66c614f8234a3837381ed69d547b48bd8fd32966bf8f62bef8b2f
  - path: src/state.rs
    sha256: 7b8fec453bb1dfe5c1f97dc4074e77a20822db24e2b2231eb0841fd5133f808e
- question: What is the eval tool and how does it differ from the other AI tools? What are its limitations compared to Python?
  anchor: '#what-is-the-eval-tool-and-how-does-it-differ-from-the-other-ai-tools-what-are-its-limitations-compared-to-python'
  generated_at: 2026-03-05T19:22:42.615717200Z
  sources:
  - path: README.md
    sha256: 48619761618f4eb9c6a8752861b56d060f3cf3ac9e65c12c26efde7beeea3b89
  - path: src/ai.rs
    sha256: bf64a29c52c3c2b49f1be78f700441e9fb38be91ff0446eda369ff157e93c88b
  - path: src/eval.rs
    sha256: 8be069f1dbe8b14f8aff49fde611025d908bc208b0221f8f57a3a56f8bdbb4d2
- question: How are multiple questions answered concurrently, and what controls the parallelism?
  anchor: '#how-are-multiple-questions-answered-concurrently-and-what-controls-the-parallelism'
  generated_at: 2026-03-05T19:22:42.615717200Z
  sources:
  - path: src/ai.rs
    sha256: bf64a29c52c3c2b49f1be78f700441e9fb38be91ff0446eda369ff157e93c88b
  - path: src/copilot/client.rs
    sha256: a2d9086a8131e1daece002083edaaee59aca50f74a7e2832d970c94bb903f050
  - path: src/copilot/jsonrpc.rs
    sha256: 6a99b8c156c39fe4fdecddc4e840e3d0d5db573f6463b74b0cf0965fb4fe0777
  - path: src/copilot/session.rs
    sha256: b7044f3b44e6104fa30428cc065bdc8ccdc15ecf07c50a2831352087365f995a
  - path: src/main.rs
    sha256: d359bd38fc73a0392ccaea2e257733d6c05b0180d9b710405ad3d412b9dbd32b
  - path: src/orchestrator.rs
    sha256: b8867fbb95e66c614f8234a3837381ed69d547b48bd8fd32966bf8f62bef8b2f
- question: What prompts are given to Copilot for generating answers?
  anchor: '#what-prompts-are-given-to-copilot-for-generating-answers'
  generated_at: 2026-03-05T19:22:42.615717200Z
  sources:
  - path: src/ai.rs
    sha256: bf64a29c52c3c2b49f1be78f700441e9fb38be91ff0446eda369ff157e93c88b
  - path: src/config.rs
    sha256: d6ec013fdc917e4131a90dcca1cf868544bfe7cb460bb15eaf7901202cf97753
  - path: src/copilot/types.rs
    sha256: 3ea65a2ccc4baf35afa690880cde5767a0aec98be9c5d97f7205d80320b7e1d4
- question: how does faqifai optimize token and request usage?
  anchor: '#how-does-faqifai-optimize-token-and-request-usage'
  generated_at: 2026-03-05T19:22:42.615717200Z
  sources:
  - path: src/ai.rs
    sha256: bf64a29c52c3c2b49f1be78f700441e9fb38be91ff0446eda369ff157e93c88b
  - path: src/codebase.rs
    sha256: e8a9bc3f9e9fd38efe4bec8deec05679c70b00e16948e2e227427c3e24612c71
  - path: src/copilot/client.rs
    sha256: a2d9086a8131e1daece002083edaaee59aca50f74a7e2832d970c94bb903f050
  - path: src/copilot/session.rs
    sha256: b7044f3b44e6104fa30428cc065bdc8ccdc15ecf07c50a2831352087365f995a
  - path: src/orchestrator.rs
    sha256: b8867fbb95e66c614f8234a3837381ed69d547b48bd8fd32966bf8f62bef8b2f
  - path: src/output.rs
    sha256: 46d91c93c91a000e4e45de966a52fe52ce2bcb749e673749ce2843abedff5aba
  - path: src/state.rs
    sha256: 7b8fec453bb1dfe5c1f97dc4074e77a20822db24e2b2231eb0841fd5133f808e
- question: What models does faqifai use?
  anchor: '#what-models-does-faqifai-use'
  generated_at: 2026-03-05T19:22:42.615717200Z
  sources:
  - path: src/ai.rs
    sha256: bf64a29c52c3c2b49f1be78f700441e9fb38be91ff0446eda369ff157e93c88b
  - path: src/copilot/types.rs
    sha256: 3ea65a2ccc4baf35afa690880cde5767a0aec98be9c5d97f7205d80320b7e1d4
  - path: src/main.rs
    sha256: d359bd38fc73a0392ccaea2e257733d6c05b0180d9b710405ad3d412b9dbd32b
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

Perfect! I have all the information I need. Let me create the comprehensive answer:

## Security Model

All AI file access is secured by a **single choke point**: the `safe_join` function in [codebase.rs:496-535](./src/codebase.rs#L496-L535). Every file operation—whether from custom Starlark scripts or built-in Copilot tools—passes through this validation layer.

### Core Security Function: `safe_join`

The `safe_join` function enforces workspace containment through multiple defense layers:

**1. Path Format Validation** ([codebase.rs:499-518](./src/codebase.rs#L499-L518))
- **Null bytes**: Rejected to prevent path injection attacks
- **Absolute paths**: Blocked for Unix (`/`), Windows (`C:`), URI schemes (`file://`), and tilde expansion (`~`)
- **Path traversal**: Any `..` component is rejected before path construction
- **Empty paths**: Normalized to `.` (workspace root)

**2. Canonicalization & Containment Check** ([codebase.rs:526-532](./src/codebase.rs#L526-L532))
```rust
let canonical_root = root.canonicalize()?;
let canonical_full = full.canonicalize()
    .map_err(|_| anyhow::anyhow!("Path not found: {}", relative))?;

if !canonical_full.starts_with(&canonical_root) {
    anyhow::bail!("Path escapes workspace: {}", relative);
}
```

This final check resolves symlinks and ensures the real file location is within the workspace, preventing:
- Symlink attacks that point outside the workspace
- OS-specific path tricks that bypass string-based validation
- Race conditions where files are replaced after validation

### Tool Integration Points

**Custom Tool (Starlark)** — [eval.rs:64-121](./src/eval.rs#L64-L121)

All Starlark file I/O functions (`read_file`, `read_file_lines`, `grep`, `find_files`, `list_dir`) receive paths as arguments and pass them through `safe_join` before accessing disk. The `EvalContext` holds the workspace root, ensuring scripts cannot override it.

Test coverage: [eval.rs:309-312](./src/eval.rs#L309-L312) verifies that `read_file("../../../etc/passwd")` fails with an error.

**Built-in Copilot Tools** — [ai.rs:307-322](./src/ai.rs#L307-L322)

The GitHub Copilot CLI is spawned with `working_directory` set to the repository root ([client.rs:40-42](./src/copilot/client.rs#L40-L42)). The CLI's built-in tools (`view`, `grep`, `glob`) use this as their base directory and cannot access files outside it—this is enforced by the Copilot CLI itself, not faqifai code.

Write tools (`edit_file`, `create_file`, `delete_file`) are explicitly excluded from AI sessions ([ai.rs:270-279](./src/ai.rs#L270-L279)):
```rust
fn excluded_tools() -> Vec<String> {
    ["edit_file", "create_file", "delete_file"]
        .iter().map(|s| s.to_string()).collect()
}
```

### Additional Defenses in Walk Operations

File discovery operations (`walk_glob`, `walk_dir`) that scan directories include redundant containment checks ([codebase.rs:77-79](./src/codebase.rs#L77-L79), [112-114](./src/codebase.rs#L112-L114)):
```rust
if let Ok(canonical) = path.canonicalize() {
    if !canonical.starts_with(&canonical_root) { continue; }
}
```

This ensures that even if the filesystem walker encounters symlinks or mounted filesystems, only workspace-contained files are processed.

### Test Coverage

[codebase.rs:547-576](./src/codebase.rs#L547-L576) includes comprehensive security tests:
- `safe_join_rejects_absolute_unix`: Blocks `/etc/passwd`
- `safe_join_rejects_absolute_windows`: Blocks `C:\Windows\System32`
- `safe_join_rejects_dot_dot`: Blocks `../../../etc/passwd` and `src/../../secret`
- `safe_join_rejects_tilde`: Blocks `~/.ssh/id_rsa`
- `safe_join_rejects_null_byte`: Blocks null byte injection
- `safe_join_rejects_uri`: Blocks `file:///etc/passwd`
- Valid relative paths (`Cargo.toml`, `src/main.rs`) are allowed

# How does the tool decide whether a previously generated answer is stale and needs regeneration?

Now I have enough context. Let me write the answer documenting the staleness detection mechanism.

The staleness detection mechanism in faqifai works as follows:

## Detection triggers

An answer is considered stale and needs regeneration when any of three conditions are met:

1. **Never generated** — The output file doesn't exist or doesn't contain an entry for the question
2. **TTL expired** — A time-to-live is configured and has elapsed since generation
3. **Sources changed** — One or more source files tracked in the answer's metadata have changed content

## How it works

### 1. Source tracking during generation

When the AI generates an answer, it appends a special comment block listing all files it actually read ([ai.rs:51-60](./src/ai.rs#L51-L60)):

```html
<!-- faqifai-sources
src/path/to/file.rs
src/path/to/other.rs
-->
```

The orchestrator extracts these paths from the answer ([ai.rs:140-158](./src/ai.rs#L140-L158)), computes their SHA-256 hashes, and stores them in the output file's frontmatter as a list of `SourceFile` entries ([state.rs:27-32](./state.rs#L27-L32)):

```rust
pub struct SourceFile {
    pub path: String,
    pub sha256: String,
}
```

### 2. Staleness checking

When running `faqifai run` or `faqifai status`, the tool calls `state::check_staleness()` for each question ([state.rs:100-152](./state.rs#L100-L152)):

**Step 1: Load frontmatter**  
Parse the output markdown file to extract the YAML frontmatter containing the TOC ([state.rs:48-62](./state.rs#L48-L62)). If the file or question entry doesn't exist, return `Staleness::NeverGenerated`.

**Step 2: Check TTL (if configured)**  
If a TTL like `"7d"` or `"24h"` is set, compare the question's `generated_at` timestamp against the current time ([state.rs:122-129](./state.rs#L122-L129)). If the age exceeds the TTL duration, return `Staleness::TtlExpired`.

**Step 3: Verify source hashes**  
For each `SourceFile` in the TOC entry, recompute the current hash and compare ([state.rs:132-149](./state.rs#L132-L149)):

- **Files**: Use `codebase::hash_source_cached()` which computes SHA-256 of file content
- **Directories** (paths ending in `/`): Compute a merkle tree hash of all contained files ([codebase.rs:40-54](./codebase.rs#L40-L54))
- **Glob patterns** (containing `*`, `?`, `[`, `{`): Walk matching files and compute a merkle tree hash ([codebase.rs:36-38](./codebase.rs#L36-L38), [codebase.rs:63-96](./codebase.rs#L63-L96))

The merkle hash aggregates `(path, hash)` pairs in sorted order, so it detects:
- Content changes in existing files
- New files added to a directory/pattern
- Files deleted from a directory/pattern

If any hash mismatch is found, the changed paths are collected and returned as `Staleness::SourcesChanged(Vec<String>)`.

**Step 4: Return status**  
If all checks pass, return `Staleness::Fresh` ([state.rs:151](./state.rs#L151)).

### 3. Regeneration behavior

In [orchestrator.rs:74-108](./orchestrator.rs#L74-L108), only questions with non-fresh staleness are sent to the AI. The orchestrator passes:

- The previous answer for context ([orchestrator.rs:124](./orchestrator.rs#L124))
- A list of changed source paths so the AI knows where to focus ([orchestrator.rs:117-121](./orchestrator.rs#L117-L121))

The AI can respond with `NO_UPDATE_NEEDED` if after verification it determines the previous answer is still accurate ([ai.rs:14](./ai.rs#L14), [ai.rs:336-347](./ai.rs#L336-L347)). Otherwise, it generates a new answer with an updated sources block.

### 4. Hash caching

To avoid redundant I/O when checking multiple questions that reference the same files, the orchestrator maintains a `HashMap<String, String>` cache that's shared across all staleness checks in a single run ([orchestrator.rs:24](./orchestrator.rs#L24), [state.rs:105](./state.rs#L105), [codebase.rs:215-219](./codebase.rs#L215-L219)).

## Important details

- **Fallback sources**: If the AI doesn't emit a sources block, the system falls back to the scope/hints specified in the `.faq` file for staleness tracking ([ai.rs:351-364](./ai.rs#L351-L364))
- **Directory notation**: Listing `src/some/dir/` (with trailing slash) tracks all files under that directory. Adding/removing files triggers regeneration ([codebase.rs:226-233](./codebase.rs#L226-L233))
- **Glob support**: Patterns like `src/**/*.rs` are detected by checking for glob metacharacters and tracked using a merkle hash of all matching files ([codebase.rs:36-38](./codebase.rs#L36-L38), [codebase.rs:219-224](./codebase.rs#L219-L224))
- **Error handling**: If a tracked file is deleted or becomes unreadable, it's treated as a change and triggers regeneration ([state.rs:140-143](./state.rs#L140-L143))

# What is the eval tool and how does it differ from the other AI tools? What are its limitations compared to Python?

Perfect! Now I have all the information I need. Let me compile the comprehensive answer:

## The `analyze` tool

The `analyze` tool is a custom AI tool (not built into GitHub Copilot) that executes **sandboxed Starlark scripts** for multi-step codebase analysis. It's registered in [ai.rs:245-267](./src/ai.rs#L245-L267) and implemented in [eval.rs](./src/eval.rs).

### Purpose

The tool lets the AI perform complex, multi-step analysis in **a single tool call** instead of making 3+ sequential calls to built-in read/grep tools. This is explicitly recommended in the system prompt ([ai.rs:41](./src/ai.rs#L41)):

> **Use `analyze` instead of making 3 or more sequential read/grep tool calls.** If you need to read multiple files, cross-reference data, aggregate results, or parse structured files (TOML, JSON), write an `analyze` script to do it in one call.

### How it differs from other AI tools

| Aspect | `analyze` tool | Built-in Copilot tools |
|--------|---------------|----------------------|
| **Implementation** | Custom tool, implemented in Rust ([eval.rs](./src/eval.rs)) | Built into GitHub Copilot CLI |
| **Registration** | Dynamically registered on each session ([ai.rs:324-395](./src/ai.rs#L324-L395)) | Always available (except excluded ones) |
| **Language** | Starlark scripting language | Individual function calls |
| **Composition** | Multi-step scripts with variables, loops, logic | Single atomic operations |
| **Data processing** | JSON/TOML parsing, regex, cross-referencing | Read-only access to raw text |
| **Token efficiency** | One tool call for complex analysis | Multiple round trips |

Built-in tools available ([README.md:9](./README.md#L9), [ai.rs:309](./src/ai.rs#L309)):
- File reading (view individual files)
- Search/grep across files
- File listing/navigation
- ~~`edit_file`, `create_file`, `delete_file`~~ (explicitly excluded in [ai.rs:270-279](./src/ai.rs#L270-L279))

### Starlark functions provided

The tool exposes a sandboxed environment with these capabilities ([eval.rs:61-168](./src/eval.rs#L61-L168)):

**File I/O** (all paths relative to workspace root):
- `read_file(path)` — Read entire file ([eval.rs:64-71](./src/eval.rs#L64-L71))
- `read_file_lines(path, start, end)` — Read line range with line numbers ([eval.rs:74-84](./src/eval.rs#L74-L84))
- `find_files(pattern)` — Glob file search ([eval.rs:100-112](./src/eval.rs#L100-L112))
- `list_dir(path)` — List directory contents ([eval.rs:115-121](./src/eval.rs#L115-L121))
- `grep(pattern, glob, max_results)` — Regex search across files ([eval.rs:88-97](./src/eval.rs#L88-L97))

**Text processing**:
- `lines(text)` — Split into list ([eval.rs:124-126](./src/eval.rs#L124-L126))
- `regex_find(pattern, text)` — Extract matches ([eval.rs:146-157](./src/eval.rs#L146-L157))
- `regex_match(pattern, text)` — Boolean test ([eval.rs:160-167](./src/eval.rs#L160-L167))

**Data parsing**:
- `json_parse(text)` — Parse JSON to Starlark dict/list ([eval.rs:129-135](./src/eval.rs#L129-L135))
- `toml_parse(text)` — Parse TOML to Starlark dict ([eval.rs:138-144](./src/eval.rs#L138-L144))

### Limitations compared to Python

Starlark is **Python-like but NOT Python**. The system prompt ([ai.rs:84-93](./src/ai.rs#L84-L93)) and dialect configuration ([eval.rs:37-41](./src/eval.rs#L37-L41)) enforce these restrictions:

**Missing language features**:
- ❌ `import`, `class`, `try`/`except`, `while`, `set`, `with`, `yield`, `global`, `nonlocal`
- ❌ No exception handling — errors propagate immediately
- ✅ Supports: `for`, `if`/`elif`/`else`, `def`, list/dict comprehensions, `lambda`

**f-string restrictions** ([eval.rs:39](./src/eval.rs#L39)):
- Only simple variable names: `f"value is {x}"` ✅
- No expressions in braces: `f"{d['key']}"` ❌
- Use concatenation instead: `name + ": " + str(d["key"])`

**Limited builtins**:
- Standard functions: `len`, `range`, `enumerate`, `zip`, `sorted`, `reversed`, `str`, `int`, `float`, `bool`, `list`, `dict`, `type`, `hasattr`, `getattr`, `repr`, `min`, `max`, `any`, `all`
- String methods: `.split()`, `.strip()`, `.startswith()`, `.endswith()`, `.replace()`, `.upper()`, `.lower()`, `.find()`, `.count()`, `.join()`, `.format()`
- List methods: `.append()`, `.extend()`, `.insert()`, `.pop()`, `.remove()`, `.index()`
- Dict methods: `.get()`, `.keys()`, `.values()`, `.items()`, `.pop()`, `.update()`

**Sandbox restrictions** ([README.md:124](./README.md#L124), [eval.rs:15-20](./src/eval.rs#L15-L20)):
- ❌ No network access
- ❌ No shell execution
- ❌ No file writes
- ❌ No path traversal outside workspace ([eval.rs:309-312](./src/eval.rs#L309-L312) test case)
- ✅ Read-only codebase access via provided functions

**Integer precision**:
- JSON/TOML integers are cast to `i32` ([eval.rs:177](./src/eval.rs#L177), [eval.rs:201](./src/eval.rs#L201)), losing precision for large values

### Use cases

Example patterns from tests and system prompt:
- Cross-reference analysis (find function definitions and call sites) — [eval.rs:296-306](./src/eval.rs#L296-L306)
- Dependency extraction from `Cargo.toml`/`package.json` — [eval.rs:352-368](./src/eval.rs#L352-L368)
- Multi-file aggregation with filtering — [ai.rs:97-130](./src/ai.rs#L97-L130)

# How are multiple questions answered concurrently, and what controls the parallelism?

Perfect! Now I have all the information I need. Let me create a comprehensive answer.

## Concurrent Processing Architecture

Multiple questions are answered concurrently using **Tokio async tasks** with a **shared work queue** pattern. The parallelism is controlled by the `--concurrency` flag (default: 4).

### Concurrency Flow

1. **Orchestrator collects stale questions** — [orchestrator.rs:90-143](./src/orchestrator.rs#L90-L143) identifies questions needing regeneration across all `.faq` files and builds a vector of `QuestionInput` structs.

2. **Work is dispatched to worker pool** — [orchestrator.rs:167](./src/orchestrator.rs#L167) calls `ai::answer_questions_concurrent()` with the questions list and concurrency limit.

3. **Worker tasks process questions concurrently** — [ai.rs:418-491](./src/ai.rs#L418-L491) implements the worker pool:
   - Creates a **single shared Copilot process** ([ai.rs:428-431](./src/ai.rs#L428-L431)) wrapped in `Arc<Client>`
   - Spawns `min(concurrency, total_questions)` worker tasks ([ai.rs:444-448](./src/ai.rs#L444-L448))
   - Each worker pulls questions from a shared `Mutex<Vec<(usize, QuestionInput)>>` queue ([ai.rs:437-439](./src/ai.rs#L437-L439))
   - Workers call `answer_question()` ([ai.rs:465-471](./src/ai.rs#L465-L471)) which creates a **fresh session per question** to avoid conversation history bloat

4. **Results are sorted back to original order** — [ai.rs:485-490](./src/ai.rs#L485-L490) uses the indexed queue to restore question order.

### Single Process, Multiple Sessions

The key design is **one Copilot CLI process shared by all workers** ([ai.rs:428-431](./src/ai.rs#L428-L431)). Each worker creates a new session via `client.create_session()` ([ai.rs:310-322](./src/ai.rs#L310-L322)) for each question, then destroys it ([ai.rs:331-333](./src/ai.rs#L331-L333)). This avoids:
- Spawning N copilot processes (expensive)
- Conversation history accumulation within a session
- Tool registration conflicts between questions

The single Copilot process multiplexes all sessions through a **JSON-RPC transport** ([copilot/jsonrpc.rs:54-100](./src/copilot/jsonrpc.rs#L54-L100)) over stdio pipes. The transport manages:
- Request/response correlation via JSON-RPC IDs
- Async notification dispatch for tool calls and session events
- Thread-safe read/write to the child process stdin/stdout

### Parallelism Control

Controlled at **three points**:

1. **CLI flag** — [main.rs:19-21](./src/main.rs#L19-L21) defines `--concurrency` with default of 4:
   ```rust
   #[arg(long, default_value_t = 4, global = true)]
   concurrency: usize,
   ```

2. **Passed to orchestrator** — [main.rs:101](./src/main.rs#L101) passes `cli.concurrency` to `orchestrator::run()`.

3. **Worker pool size** — [ai.rs:444-445](./src/ai.rs#L444-L445) spawns exactly `concurrency.min(total)` tasks:
   ```rust
   let workers = concurrency.min(total);
   ```

### Example Flow (concurrency=4, 10 questions)

```
Copilot Process (single)
     ↓ JSON-RPC transport
     ├─ Worker 0 → Session A (Q1) → destroy → Session B (Q5) → destroy → Session C (Q9)
     ├─ Worker 1 → Session D (Q2) → destroy → Session E (Q6) → destroy → Session F (Q10)
     ├─ Worker 2 → Session G (Q3) → destroy → Session H (Q7) → destroy
     └─ Worker 3 → Session I (Q4) → destroy → Session J (Q8) → destroy
```

Each worker pulls from the shared queue until empty, processing multiple questions sequentially while other workers run in parallel.

# What prompts are given to Copilot for generating answers?

Perfect! Now I have a comprehensive understanding of the prompts. Let me create the answer:

## System Prompt

The main system prompt is defined in [`ai.rs:19-130`](./src/ai.rs#L19-L130) as the constant `SYSTEM_PROMPT`. This prompt:

- **Instructs the AI to act as a codebase research agent** generating reference documentation for consumption by other AI agents and developers
- **Enforces thoroughness**: trace call chains, follow imports, read config files, surface edge cases and error paths
- **Mandates factual, unopinionated answers**: state what code does, not what it should do; never speculate
- **Requires source file references**: every claim must be traceable using markdown links with relative paths
- **Encourages aggressive tool use**: grep, file navigation, search, with specific workflow guidance
- **Provides Starlark scripting reference**: complete documentation for the `analyze` tool including built-in functions, language notes, and examples
- **Specifies output structure**: scannable format with sub-headings, bullet points, minimal prose
- **Handles answer regeneration**: when previous answers are provided, verify against current code and return `NO_UPDATE_NEEDED` if still accurate
- **Requires sources block**: append `<!-- faqifai-sources ... -->` listing files that influenced the answer

The system prompt is **dynamically customized per question** by [`build_system_prompt`](./src/ai.rs#L174-L177), which replaces the `{output_dir}` placeholder with a relative path prefix (e.g., `../../`) based on the output file's location, ensuring relative links work correctly.

The system message mode is set to **`"replace"`** ([`ai.rs:315`](./src/ai.rs#L315)), completely replacing Copilot's default system prompt.

## User Message Structure

The user message is constructed by [`build_user_message`](./src/ai.rs#L180-L242) and includes up to five sections:

1. **Context** (optional): Combined from `.faq` file's top-level `context` and per-question `context` fields ([`config.rs:77-84`](./src/config.rs#L77-L84))

2. **Suggested Starting Points** (optional): List of hint file paths from the question's `hints` field ([`ai.rs:196-203`](./src/ai.rs#L196-L203))

3. **Previous Answer** (optional, for regeneration): When regenerating stale answers, includes:
   - Notice that previous answer may be outdated and must be verified
   - **Detected changes** list showing which tracked sources changed ([`ai.rs:214-219`](./src/ai.rs#L214-L219))
   - **Files previously tracked** list from the sources block ([`ai.rs:222-229`](./src/ai.rs#L222-L229))
   - The full previous answer wrapped in `<previous_answer>` tags ([`ai.rs:232-234`](./src/ai.rs#L232-L234))

4. **Question**: The actual question text ([`ai.rs:238-239`](./src/ai.rs#L238-L239))

## Tools Configuration

The session is configured with ([`ai.rs:310-322`](./src/ai.rs#L310-L322)):

### Custom Tools
One custom tool definition ([`ai.rs:245-267`](./src/ai.rs#L245-L267)):
- **`analyze`**: Executes Starlark scripts for multi-step codebase analysis, with parameters:
  - `intent`: One-line description of what the script does
  - `script`: Starlark code to execute (returns last expression as string)

### Excluded Built-in Tools  
Write capabilities are explicitly blocked ([`ai.rs:270-279`](./src/ai.rs#L270-L279)):
- `edit_file`
- `create_file`  
- `delete_file`

All other Copilot CLI built-in tools (read/search tools like grep, glob, view) remain available.

## Session Configuration

Each question gets a fresh session ([`ai.rs:310-322`](./src/ai.rs#L310-L322)) with:
- **Model**: Specified via CLI (defaults to current Copilot default)
- **Working directory**: Repository root path
- **System message**: The customized system prompt with `mode: "replace"`
- **Tools**: Custom tool definitions + excluded tools list

The tool handler for `analyze` is registered at [`ai.rs:378-395`](./src/ai.rs#L378-L395), which invokes the Starlark evaluator from [`eval`](./src/eval.rs).

# how does faqifai optimize token and request usage?

Now I have a comprehensive view of the optimization strategies. Let me create the answer with proper file references.

## Caching and Staleness Detection

faqifai minimizes regeneration by tracking file content hashes and comparing them against previously generated answers. Each output file stores a YAML frontmatter block containing source file paths and their SHA-256 hashes ([state.rs:27-32](./src/state.rs#L27-L32)):

```rust
pub struct SourceFile {
    pub path: String,
    pub sha256: String,
}
```

When checking staleness, [state.rs:100-152](./src/state.rs#L100-L152) implements three checks:
1. **Never generated** — no previous output exists
2. **TTL expired** — answer age exceeds configured TTL (if set)
3. **Sources changed** — SHA-256 hashes of tracked files differ from stored values

Only stale questions trigger AI requests ([orchestrator.rs:98-100](./src/orchestrator.rs#L98-L100)).

## Hash Cache for Deduplication

A shared `HashMap<String, String>` cache is created once per run ([orchestrator.rs:23-24](./src/orchestrator.rs#L23-L24)) and passed to all staleness checks. This prevents rehashing the same file/glob when multiple questions reference it:

```rust
let mut hash_cache: std::collections::HashMap<String, String> = 
    std::collections::HashMap::new();
```

The cache is used in both staleness checking ([orchestrator.rs:77](./src/orchestrator.rs#L77)) and context collection ([orchestrator.rs:110-115](./src/orchestrator.rs#L110-L115)).

## Merkle Tree Hashing for Directories and Globs

Rather than storing individual file hashes for directories or glob patterns (which would cause false staleness when unrelated files change), faqifai uses merkle tree hashing ([codebase.rs:40-54](./src/codebase.rs#L40-L54)):

```rust
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
```

This single hash represents the entire directory or glob result ([codebase.rs:131-182](./src/codebase.rs#L131-L182)). Adding, removing, or modifying any file within the scope changes the merkle hash, triggering regeneration.

## NO_UPDATE_NEEDED Sentinel

When regenerating a stale answer, faqifai provides the previous answer to the AI along with a list of changed files ([ai.rs:180-242](./src/ai.rs#L180-L242)). The system prompt instructs the AI to verify whether the answer is still accurate and respond with `NO_UPDATE_NEEDED` if nothing has changed ([ai.rs:49](./src/ai.rs#L49)):

> **If after thorough verification the previous answer is still fully accurate and complete, respond with exactly `NO_UPDATE_NEEDED` (nothing else).**

When this sentinel is detected ([ai.rs:336-347](./src/ai.rs#L336-L347)), faqifai reuses the previous answer without generating new content — saving both tokens and time.

## Session-Per-Question Architecture

Each question gets a fresh copilot session ([ai.rs:310-322](./src/ai.rs#L310-L322)) rather than reusing a single long-lived session. This prevents conversation history bloat:

```rust
let session = client
    .create_session(SessionConfig {
        model: Some(model.to_string()),
        system_message: Some(SystemMessageConfig {
            mode: "replace".to_string(),
            content: system_prompt,
        }),
        // ...
    })
    .await?;
```

After answering, each session is immediately destroyed ([ai.rs:331-333](./src/ai.rs#L331-L333)). This keeps token overhead minimal — each question starts with only the system prompt and the question itself, not the entire history of previous questions.

## Shared Copilot Process with Concurrent Workers

Rather than spawning one copilot process per question, faqifai creates a single copilot process ([ai.rs:428-431](./src/ai.rs#L428-L431)) and shares it across concurrent worker tasks ([ai.rs:422-491](./src/ai.rs#L422-L491)):

```rust
let client = Arc::new(Client::new(ClientOptions {
    working_directory: Some(root.to_path_buf()),
    ..Default::default()
}).await?);
```

Workers pull from a shared queue ([ai.rs:436-443](./src/ai.rs#L436-L443)) and create lightweight sessions on the shared client. The `Arc<Client>` is cloned for each worker but points to the same underlying process ([ai.rs:452](./src/ai.rs#L452)), avoiding process spawn overhead and reducing memory footprint.

## No File Content in Prompts

Hint paths and scope patterns are used only for staleness tracking ([codebase.rs:131-182](./src/codebase.rs#L131-L182)). File content is **never** injected into the user message. The AI uses its own file exploration tools to read only the files it needs ([ai.rs:245-267](./src/ai.rs#L245-L267)).

This keeps prompt sizes minimal and gives the AI control over what it reads.

## AI-Reported Source Tracking

After answering, faqifai extracts the `<!-- faqifai-sources ... -->` block from the AI's response ([ai.rs:140-158](./src/ai.rs#L140-L158)) and rehashes only those files ([ai.rs:351-368](./src/ai.rs#L351-L368)):

```rust
let (cleaned, ai_source_paths) = extract_ai_sources(&answer);
let source_map = if !ai_source_paths.is_empty() {
    let mut map = std::collections::HashMap::new();
    for path in &ai_source_paths {
        match codebase::hash_source(root, path) {
            Ok(hash) => { map.insert(path.clone(), hash); }
            // ...
        }
    }
    map
} else {
    sources  // fallback to scope/hints
};
```

This allows the AI to narrow the staleness scope to only the files it actually read, reducing false positives (e.g., if a wide scope was provided but the AI only needed a subset).

## Excluded Write Tools

All file mutation tools (`edit_file`, `create_file`, `delete_file`) are explicitly excluded from the AI's available toolset ([ai.rs:270-279](./src/ai.rs#L270-L279)). This prevents accidental side effects and ensures the AI operates in read-only mode, reducing error handling complexity and keeping sessions lightweight.

## Incremental Regeneration Per Output File

When multiple questions share an output file, faqifai merges fresh answers (unchanged since last run) with newly regenerated answers ([orchestrator.rs:192-217](./src/orchestrator.rs#L192-L217)). Only stale questions are sent to the AI. The frontmatter preserves `generated_at` timestamps and source hashes individually per question ([state.rs:18-25](./src/state.rs#L18-L25)), enabling fine-grained staleness detection.

# What models does faqifai use?

Alright! I have all the information I need to answer the question comprehensively.

## Default model

**claude-sonnet-4.6** is the default model used by faqifai. This is configured in [main.rs:24](./src/main.rs#L24) as the default value for the `--model` CLI argument.

## Model override

The model can be overridden at runtime using the `--model` flag, which is a global argument accepted by all commands:

```sh
faqifai run --model gpt-5.2-codex
```

## Model usage flow

1. The `--model` argument from [main.rs:24-25](./src/main.rs#L24-L25) is captured globally
2. It is passed to `orchestrator::run()` in [main.rs:101](./src/main.rs#L101)
3. The orchestrator forwards it to `ai::answer_questions_concurrent()` in [ai.rs:426](./src/ai.rs#L426)
4. Each worker passes it to `ai::answer_question()` in [ai.rs:286](./src/ai.rs#L286)
5. Finally, it's used to configure the Copilot session via `SessionConfig` in [ai.rs:311-312](./src/ai.rs#L311-L312)

## Model compatibility

faqifai accepts any model string and passes it directly to the GitHub Copilot CLI. The Copilot SDK protocol supports a `model` field in the session configuration ([types.rs:13](./src/copilot/types.rs#L13)), so any model identifier supported by the Copilot CLI should work.