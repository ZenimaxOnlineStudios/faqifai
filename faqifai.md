---
generated_by: faqifai
source: faqifai.faq
toc:
- question: How does the JSON-RPC transport work between faqifai and the Copilot CLI, and what framing protocol is used?
  anchor: '#how-does-the-json-rpc-transport-work-between-faqifai-and-the-copilot-cli-and-what-framing-protocol-is-used'
  generated_at: 2026-03-09T19:23:47.193673400Z
  sources:
  - path: src/copilot/client.rs
    sha256: a2d9086a8131e1daece002083edaaee59aca50f74a7e2832d970c94bb903f050
  - path: src/copilot/jsonrpc.rs
    sha256: 65604690c51819f7be5331fb85bc0316b70663574370c420ff5df54a35009387
  - path: src/copilot/mod.rs
    sha256: 1bb17abf43d0341ea37dfaacbe8687c1878e98e8d199c1169d7c2df38c550894
- question: What security measures prevent the AI's tool calls from accessing files outside the workspace?
  anchor: '#what-security-measures-prevent-the-ais-tool-calls-from-accessing-files-outside-the-workspace'
  generated_at: 2026-03-09T19:23:47.193673400Z
  sources:
  - path: src/ai.rs
    sha256: 3626ad5277449405544a0ca18ca0a965715959bc1b2a8c5ecadae35061e77b86
    lines:
      start: 313
      end: 324
      content_len: 246
  - path: src/codebase.rs
    sha256: 760a7bcb285d8daf1fa2ec7628909746d2721a137a5d8ecf442f1a304ad28d50
  - path: src/eval.rs
    sha256: 8be069f1dbe8b14f8aff49fde611025d908bc208b0221f8f57a3a56f8bdbb4d2
- question: How does the tool decide whether a previously generated answer is stale and needs regeneration?
  anchor: '#how-does-the-tool-decide-whether-a-previously-generated-answer-is-stale-and-needs-regeneration'
  generated_at: 2026-03-09T19:23:47.193673400Z
  sources:
  - path: src/ai.rs
    sha256: 74f1918a5b1f46cbf2ac1dce01190bd7003ce8fb3cedcde59f1b9e6cb5a24668
  - path: src/codebase.rs
    sha256: 760a7bcb285d8daf1fa2ec7628909746d2721a137a5d8ecf442f1a304ad28d50
  - path: src/orchestrator.rs
    sha256: e48ad952ae7cf086ae8bfed543ba549f3f3f086a2660f804b503a672746125ef
  - path: src/state.rs
    sha256: 0aae09e44a95274832dafd9af8284065a8d8e69d71044042a21d2912159202b5
- question: What is the eval tool and how does it differ from the other AI tools? What are its limitations compared to Python?
  anchor: '#what-is-the-eval-tool-and-how-does-it-differ-from-the-other-ai-tools-what-are-its-limitations-compared-to-python'
  generated_at: 2026-03-09T19:23:47.193673400Z
  sources:
  - path: src/ai.rs
    sha256: 9677d8208bf20d804de9dafc72be8e9ea59aa9fc7926b0ae516f201cbe535468
    lines:
      start: 60
      end: 100
      content_len: 4726
  - path: src/codebase.rs
    sha256: b95b15229a25ee069b2c3c2bd959c2a437f29a122135d911ce478b2c9ad0857f
    lines:
      start: 179
      end: 179
      content_len: 80
  - path: src/eval.rs
    sha256: 8be069f1dbe8b14f8aff49fde611025d908bc208b0221f8f57a3a56f8bdbb4d2
- question: How are multiple questions answered concurrently, and what controls the parallelism?
  anchor: '#how-are-multiple-questions-answered-concurrently-and-what-controls-the-parallelism'
  generated_at: 2026-03-09T19:23:47.193673400Z
  sources:
  - path: src/ai.rs
    sha256: 771599db2c90d658acfdbba92d9300a09d959c429c1234e9d3436c4c524abea5
    lines:
      start: 528
      end: 735
      content_len: 7390
  - path: src/ai.rs
    sha256: 6f25cc0439e09752f9d46323bf8c3e2d03d1e97d4f3b294270f61ff54115575a
    lines:
      start: 1
      end: 49
      content_len: 1578
  - path: src/main.rs
    sha256: a38238459ae3ff82f480bd6391e7fabe962ffedd1c202928b1d9bedc8bed68c2
  - path: src/orchestrator.rs
    sha256: 6dfc11bf0fc2f514f9737a951639e9650fcfff2200bc281f642fc3b7f29dc7e6
    lines:
      start: 1
      end: 170
      content_len: 6184
- question: What prompts are given to Copilot for generating answers?
  anchor: '#what-prompts-are-given-to-copilot-for-generating-answers'
  generated_at: 2026-03-09T19:23:47.193673400Z
  sources:
  - path: src/ai.rs
    sha256: d76f2d9d51f87700a87a1de8bdbdba270ec2d1cdae6d7416c746162cf10b1a42
    lines:
      start: 163
      end: 410
      content_len: 9728
  - path: src/ai.rs
    sha256: d1c61025746a7449a809874e64a58abf9a4d3925ca0559e795bbfd5e277e5bee
    lines:
      start: 51
      end: 325
      content_len: 14953
- question: how does faqifai optimize token and request usage?
  anchor: '#how-does-faqifai-optimize-token-and-request-usage'
  generated_at: 2026-03-09T19:23:47.193673400Z
  sources:
  - path: src/ai.rs
    sha256: 74f1918a5b1f46cbf2ac1dce01190bd7003ce8fb3cedcde59f1b9e6cb5a24668
  - path: src/codebase.rs
    sha256: 760a7bcb285d8daf1fa2ec7628909746d2721a137a5d8ecf442f1a304ad28d50
  - path: src/copilot/session.rs
    sha256: b84a0533b8f54a8f31b29c8e1674d6a0cc6e0e52b31d0836a265e8ec3ad52337
  - path: src/orchestrator.rs
    sha256: e48ad952ae7cf086ae8bfed543ba549f3f3f086a2660f804b503a672746125ef
  - path: src/state.rs
    sha256: 0aae09e44a95274832dafd9af8284065a8d8e69d71044042a21d2912159202b5
- question: What models does faqifai use?
  anchor: '#what-models-does-faqifai-use'
  generated_at: 2026-03-09T19:23:47.193673400Z
  sources:
  - path: src/ai.rs
    sha256: 1f9a5409c232d476d64410124f1090c380f20f073624dff9ea3cd32ed6b460d8
    lines:
      start: 358
      end: 362
      content_len: 208
  - path: src/main.rs
    sha256: 5deb5c956b2558203bd36f246f8d1444a2ede96bc6438e4d9e7ebd7da236d5bc
    lines:
      start: 44
      end: 50
      content_len: 255
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

## Transport Architecture

faqifai communicates with the Copilot CLI using **JSON-RPC 2.0 over standard input/output (stdio)** with **Content-Length framing**, as implemented in [jsonrpc.rs](./src/copilot/jsonrpc.rs).

The protocol is derived from the [github/copilot-sdk](https://github.com/github/copilot-sdk) Go implementation, as noted in [mod.rs:3-4](./src/copilot/mod.rs#L3-L4).

---

## Process Setup

[client.rs](./src/copilot/client.rs) spawns the Copilot CLI with `--headless --no-auto-update --log-level info --stdio` flags, piping stdin/stdout:

```rust
Command::new(&cli_path)
    .args(["--headless", "--no-auto-update", "--log-level", "info", "--stdio"])
    .stdin(std::process::Stdio::piped())
    .stdout(std::process::Stdio::piped())
    .stderr(std::process::Stdio::null())
    .kill_on_drop(true);
```

Stderr is discarded (`Stdio::null()`). The process is killed when the `Client` is dropped. After spawning, the client immediately sends a `ping` RPC to verify the connection and checks the protocol version.

The `Transport` is initialized with the stdio pipe handles ([jsonrpc.rs:65](./src/copilot/jsonrpc.rs#L65)), spawning a dedicated read-loop task for incoming messages.

---

## Framing Protocol: Content-Length Headers

Messages use **HTTP-style Content-Length framing**, identical to the Language Server Protocol (LSP).

### Writing Messages ([jsonrpc.rs:153-163](./src/copilot/jsonrpc.rs#L153-L163))

1. Serialize the JSON-RPC message to bytes
2. Write header: `Content-Length: <byte_count>\r\n\r\n`
3. Write the JSON body
4. Flush the stdin pipe

```rust
let data = serde_json::to_vec(message)?;
let header = format!("Content-Length: {}\r\n\r\n", data.len());
stdin.write_all(header.as_bytes()).await?;
stdin.write_all(&data).await?;
stdin.flush().await?;
```

### Reading Messages ([jsonrpc.rs:192-357](./src/copilot/jsonrpc.rs#L192-L357))

The read loop continuously:

1. **Reads header lines** until a blank line is encountered
2. **Extracts the `Content-Length` value** ([jsonrpc.rs:217](./src/copilot/jsonrpc.rs#L217))
3. **Reads exactly that many bytes** as the body ([jsonrpc.rs:228](./src/copilot/jsonrpc.rs#L228))
4. **Deserializes the JSON** and dispatches based on message type

```rust
if let Some(len_str) = trimmed.strip_prefix("Content-Length: ") {
    content_length = len_str.parse()?;
}
// ...
let mut body = vec![0u8; content_length];
reader.read_exact(&mut body).await?;
let raw: Value = serde_json::from_slice(&body)?;
```

**Protocol notes:**
- Headers use `\r\n` line endings; the blank separator line is also `\r\n`
- Body is read as raw bytes (not line-by-line) via `read_exact()`
- When stdout closes (0 bytes read), the loop returns `Ok(())` (EOF, [jsonrpc.rs:209](./src/copilot/jsonrpc.rs#L209))

---

## Message Flow

The transport is **bidirectional** and **fully asynchronous**.

### Client → Server (faqifai sends requests)

`Transport::request()` ([jsonrpc.rs:105](./src/copilot/jsonrpc.rs#L105)):
1. Generates a UUID as the message ID
2. Registers a `oneshot` channel in the `pending` map
3. Sends the framed JSON-RPC request
4. Awaits the oneshot receiver for the matching response

### Server → Client (Copilot CLI sends messages)

The read loop dispatches three types of incoming messages based on presence of `method` and `id` fields:

| Has `method`? | Has `id`? | Type | Handling |
|---|---|---|---|
| ✅ | ✅ | Server→client **request** (e.g., `tool.call`) | Handler from `request_handlers` map, spawned as a task; falls back to notification subscribers if no handler registered |
| ✅ | ❌ | **Notification** | Broadcast to all subscribers via unbounded channels |
| ❌ | ✅ | **Response** to a client request | Matched by string ID in `pending` map, sent via oneshot |

---

## Deadlock Prevention: `tokio::spawn` for Response Writes

A critical design detail in the read loop ([jsonrpc.rs:254](./src/copilot/jsonrpc.rs#L254)): when handling server→client requests, the response **is not written inline in the read loop**. Instead, it is written inside a `tokio::spawn`ed task ([jsonrpc.rs:264](./src/copilot/jsonrpc.rs#L264), [jsonrpc.rs:308](./src/copilot/jsonrpc.rs#L308)).

The code comment explains why:

> Clone the handler Arc and release the lock immediately so the read_loop is not blocked while the handler runs. Without this, the loop cannot drain stdout during handler execution; the subprocess's stdout pipe fills, it blocks, and our stdin write also blocks — classic pipe deadlock.

The same `tokio::spawn` pattern is used for ACK writes when a server request falls back to notification subscribers.

---

## Notification-Like Server Requests

Some Copilot CLI methods (e.g., `session.event`) are sent as **requests with an `id`** rather than true notifications ([jsonrpc.rs:294](./src/copilot/jsonrpc.rs#L294)). When no `request_handlers` entry exists for a method:

1. The params are broadcast to any `notification_subs` subscribers
2. A `null` result ACK is sent back (mirroring the Go SDK's `NotificationHandlerFor`)
3. If no subscribers exist either, a JSON-RPC `-32601` Method Not Found error is returned

---

## Key Data Structures

- **`Transport`** — wraps `Arc<Mutex<ChildStdin>>`, a `pending` map of `String → oneshot::Sender<Response>`, `request_handlers` map, and `notification_subs` map
- **`RequestHandlerFn`** — `Arc<dyn Fn(Value) -> Pin<Box<dyn Future<Output = Result<Value>> + Send>> + Send + Sync>` ([jsonrpc.rs:51-52](./src/copilot/jsonrpc.rs#L51-L52))
- **`pending`** — `HashMap<String, oneshot::Sender<Response>>` keyed by UUID string ID
- **`notification_subs`** — `HashMap<String, Vec<mpsc::UnboundedSender<Value>>>` supporting multiple subscribers per method

# What security measures prevent the AI's tool calls from accessing files outside the workspace?

The security model consists of **four defense layers** that together prevent the AI from accessing files outside the workspace.

---

## 1. Tool Exclusion

[`excluded_tools()`](./src/ai.rs#L315-L324) blocks write-capable tools provided by the GitHub Copilot SDK:

```rust
fn excluded_tools() -> Vec<String> {
    ["edit_file", "create_file", "delete_file"]
        .iter()
        .map(|s| s.to_string())
        .collect()
}
```

This list is passed to the Copilot session configuration at [ai.rs:366](./src/ai.rs#L366) and [ai.rs:565](./src/ai.rs#L565), ensuring these tools are never available to the AI. The AI only has access to read-only built-in tools plus two custom tools: `analyze` (Starlark script execution) and `mark_unchanged` (incremental update flag).

---

## 2. Working Directory Restriction

The Copilot CLI process is spawned with its working directory set to the repository root via `cmd.current_dir(cwd)` in [client.rs:40-42](./src/copilot/client.rs#L40-L42). The workspace root is passed as `working_directory` in the session config at [ai.rs:367](./src/ai.rs#L367), [ai.rs:566](./src/ai.rs#L566), and [ai.rs:654](./src/ai.rs#L654).

---

## 3. Path Validation Gate: `safe_join()`

Described in [codebase.rs:569-570](./src/codebase.rs#L569-L570) as **"the single security gate for all tool I/O"**, every file access operation funnels through [`safe_join(root, relative)`](./src/codebase.rs#L571-L610):

```rust
fn safe_join(root: &Path, relative: &str) -> Result<PathBuf>
```

### Rejection rules ([codebase.rs:572-593](./src/codebase.rs#L572-L593))

Input is first normalized by replacing `\` with `/`, then rejected if it contains:

| Check | Rule |
|---|---|
| Null bytes | Rejects paths containing `\0` |
| Unix absolute | Rejects paths starting with `/` |
| Home dir | Rejects paths starting with `~` |
| URI schemes | Rejects paths containing `://` (e.g. `file:///etc/passwd`) |
| Windows drive letters | Rejects paths where byte at index 1 is `:` (e.g. `C:\...`) |
| Path traversal | Rejects any path component equal to `..` |
| Empty path | Normalizes `""` to `"."` (workspace root) |

### Canonicalization check ([codebase.rs:600-607](./src/codebase.rs#L600-L607))

After the string-based checks, `safe_join` performs a **definitive symlink-proof check** by resolving both paths with the OS:

```rust
let canonical_root = root.canonicalize()?;
let canonical_full = full.canonicalize()
    .map_err(|_| anyhow::anyhow!("Path not found: {}", relative))?;

if !canonical_full.starts_with(&canonical_root) {
    anyhow::bail!("Path escapes workspace: {}", relative);
}
```

This catches symlink-based escapes that would bypass the string checks above.

---

## 4. Defense in Depth: Per-Operation Containment Checks + `follow_links(false)`

All file traversal operations use `ignore::WalkBuilder` with **`.follow_links(false)`** and redundant **canonicalize containment checks** on every discovered path.

| Operation | Walker config | Containment check |
|---|---|---|
| [`walk_glob`](./src/codebase.rs#L57-L90) | `follow_links(false)` | [codebase.rs:71-73](./src/codebase.rs#L71-L73) |
| [`walk_dir`](./src/codebase.rs#L93-L112) | `follow_links(false)` | [codebase.rs:106-108](./src/codebase.rs#L106-L108) |
| [`grep`](./src/codebase.rs#L348-L380) | `follow_links(false)` | [codebase.rs:376-380](./src/codebase.rs#L376-L380) |
| [`find_files`](./src/codebase.rs#L490-L509) | `follow_links(false)` | [codebase.rs:505-509](./src/codebase.rs#L505-L509) |

Even if a scanner somehow encounters an out-of-workspace path, it is silently skipped before any content is read.

---

## Tool → Security Gate Mapping

All file I/O tools exposed to the AI route through `codebase` functions:

| Tool | Entry point | Security gate |
|---|---|---|
| `read_file()` | [codebase.rs:179-184](./src/codebase.rs#L179-L184) | `safe_join` at line 180 |
| `read_file_lines()` | [codebase.rs:318-344](./src/codebase.rs#L318-L344) | `safe_join` at line 324 |
| `list_dir()` | [codebase.rs:530-567](./src/codebase.rs#L530-L567) | `safe_join` at line 531 |
| `grep()` | [codebase.rs:348-](./src/codebase.rs#L348) | Walker + containment checks |
| `find_files()` | [codebase.rs:490-](./src/codebase.rs#L490) | Walker + containment checks |

The Starlark `analyze` tool exposes these same functions via [eval.rs:64-120](./src/eval.rs#L64-L120), delegating directly to `codebase::*` with the workspace root — ensuring script-based access is equally constrained.

---

## Test Coverage

The [`mod tests`](./src/codebase.rs#L612) block validates `safe_join` rejects:

- Unix absolute paths — `"/etc/passwd"` ([codebase.rs:623-625](./src/codebase.rs#L623-L625))
- Windows absolute paths — `"C:\\Windows\\System32"` ([codebase.rs:628-630](./src/codebase.rs#L628-L630))
- Path traversal — `"../../../etc/passwd"`, `"src/../../secret"` ([codebase.rs:633-636](./src/codebase.rs#L633-L636))
- Tilde expansion — `"~/.ssh/id_rsa"` ([codebase.rs:639-641](./src/codebase.rs#L639-L641))
- Null bytes — `"src\0/evil"` ([codebase.rs:644-646](./src/codebase.rs#L644-L646))
- URI schemes — `"file:///etc/passwd"` ([codebase.rs:649-651](./src/codebase.rs#L649-L651))

And confirms valid relative paths are accepted ([codebase.rs:654-669](./src/codebase.rs#L654-L669)). [eval.rs:310](./src/eval.rs#L310) additionally tests that the Starlark `read_file` tool rejects `"../../../etc/passwd"` end-to-end.

# How does the tool decide whether a previously generated answer is stale and needs regeneration?

## Detection triggers

An answer is considered stale and needs regeneration when any of three conditions are met:

1. **Never generated** — The output file doesn't exist or has no TOC entry for the question
2. **TTL expired** — A configured TTL has elapsed since `generated_at`
3. **Sources changed** — One or more source files tracked in the answer's metadata have changed

---

## How it works

### 1. Source tracking during generation

When the AI generates an answer, it calls the `record_source` tool ([ai.rs:289-309](./src/ai.rs#L289-L309)) for each file it reads. The tool accepts:
- `path` (required) — file path relative to workspace root; trailing `/` denotes a directory
- `start_line` / `end_line` (optional) — tracks only a specific line range within the file

Recorded sources accumulate in a shared `Vec<RecordedSource>` ([ai.rs:445-464](./src/ai.rs#L445-L464)). After the AI finishes, `resolve_recorded_sources()` ([ai.rs:482-526](./src/ai.rs#L482-L526)) converts them to `SourceFile` entries:

```rust
pub struct SourceFile {
    pub path: String,
    pub sha256: String,
    pub lines: Option<LineRange>,  // set only for line-range records
}

pub struct LineRange {
    pub start: u32,
    pub end: u32,
    pub content_len: u64,  // byte length of the block at recording time
}
```

- For **whole-file / directory / glob** records: calls `codebase::hash_source()` ([ai.rs:514-521](./src/ai.rs#L514-L521))
- For **line-range** records: calls `codebase::hash_file_lines()`, which returns `(sha256, byte_len)` of the extracted block ([ai.rs:504-512](./src/ai.rs#L504-L512), [codebase.rs:239-249](./src/codebase.rs#L239-L249))
- Duplicate `(path, start_line, end_line)` entries are deduplicated, keeping the last call ([ai.rs:496-500](./src/ai.rs#L496-L500))

**Fallback**: If the AI records no sources at all, `resolve_recorded_sources` falls back to the pre-computed scope/hints hashes from `QuestionInput.source_hashes` ([ai.rs:487-493](./src/ai.rs#L487-L493)).

These `SourceFile` entries are written into the output file's YAML frontmatter TOC.

---

### 2. Staleness checking ([state.rs:112-179](./src/state.rs#L112-L179))

`state::check_staleness()` runs for each question during `faqifai run` and `faqifai status`:

**Step 1 — Load frontmatter**  
Parse the output markdown's YAML frontmatter ([state.rs:119-131](./src/state.rs#L119-L131)). If the file or TOC entry is absent → `Staleness::NeverGenerated`.

**Step 2 — Check TTL** ([state.rs:133-141](./src/state.rs#L133-L141))  
If a TTL like `"7d"` or `"24h"` is configured, compare `entry.generated_at` against `Utc::now()`. If elapsed → `Staleness::TtlExpired`.

**Step 3 — Verify source hashes** ([state.rs:143-172](./src/state.rs#L143-L172))  
For each `SourceFile` in the TOC entry, two different checks apply:

**If `source.lines` is set** (line-range record) → calls `codebase::check_line_range_staleness()` ([codebase.rs:259-315](./src/codebase.rs#L259-L315)):
1. Re-hash the same line range at the original position — if it matches, the source is **fresh**
2. If not, scan every same-sized window in the file using `content_len` as a byte-length filter to skip candidates of the wrong size. If the hash matches at any position, the source is **fresh** (the function follows moved content)
3. If no window matches → **stale**

**If `source.lines` is `None`** (whole file / directory / glob) → calls `codebase::hash_source_cached()` ([codebase.rs:209-234](./src/codebase.rs#L209-L234)):
- **Glob patterns** (contains `*`, `?`, `[`, `{`): walk matching files, compute merkle hash of sorted `(path, sha256)` pairs
- **Directory paths** (ends with `/` or resolves to a directory): same merkle tree approach
- **Plain files**: SHA-256 of file content
- Results are cached in a shared `HashMap<String, String>` to avoid rehashing across questions

The merkle tree detects content changes, additions, and deletions within a directory/glob set.

If the current hash differs, or the file is missing/unreadable → path is added to `changed` list.

**Step 4 — Return** ([state.rs:174-178](./src/state.rs#L174-L178))  
- Any changed paths → `Staleness::SourcesChanged(Vec<String>)`
- All match → `Staleness::Fresh`

---

### 3. Regeneration behavior ([orchestrator.rs:74-143](./src/orchestrator.rs#L74-L143))

Only non-fresh questions proceed to the AI. The orchestrator constructs a `QuestionInput` containing:
- `previous_answer` — existing answer text, passed as context so the AI can do differential updates ([orchestrator.rs:124](./src/orchestrator.rs#L124))
- `changed_sources` — list of changed paths extracted from `SourcesChanged`, surfaced to the AI so it knows where to focus ([orchestrator.rs:118-121](./src/orchestrator.rs#L118-L121))

The AI can call `mark_unchanged` if, after verification, the previous answer is still accurate. This sets a flag in an `unchanged_set` ([ai.rs:435-442](./src/ai.rs#L435-L442)), and the orchestrator reuses the existing answer rather than the (potentially empty) new response. Otherwise, the AI generates a new answer and records new sources.

---

### 4. Hash caching ([orchestrator.rs:24](./src/orchestrator.rs#L24))

A single `HashMap<String, String>` is shared across all staleness checks and context collection calls in one run:

```rust
let mut hash_cache: HashMap<String, String> = HashMap::new();
```

This prevents redundant SHA-256 / directory-walk / glob-walk operations when multiple questions reference the same files.

---

## Summary of staleness types

| `Staleness` variant | Trigger | Behavior |
|---|---|---|
| `NeverGenerated` | Output file missing, or no TOC entry | Full generation |
| `TtlExpired` | `generated_at + ttl < now` | Full regeneration |
| `SourcesChanged(paths)` | Hash mismatch on any tracked source | Regeneration with changed paths listed |
| `Fresh` | All checks pass | Skipped; existing answer preserved |

# What is the eval tool and how does it differ from the other AI tools? What are its limitations compared to Python?

The **analyze tool** (called "eval" in the implementation) is a sandboxed Starlark script execution environment that lets the AI perform multi-step codebase analysis in a single tool call. It is defined in [eval.rs](./src/eval.rs) and registered as a custom tool in [ai.rs](./src/ai.rs).

---

## Purpose and Design

The tool exists to reduce API roundtrips. Instead of making sequential calls to read file A, parse it, search for references, read file B, and cross-reference — the AI writes one Starlark script that does all of it and returns the aggregated result.

It is directly recommended in the AI's system prompt ([ai.rs:76](./src/ai.rs#L76)):

> Use `analyze` instead of making 3 or more sequential read/grep tool calls. If you need to read multiple files, cross-reference data, aggregate results, or parse structured files (TOML, JSON), write an `analyze` script to do it in one call.

---

## Implementation

The `evaluate()` function ([eval.rs:23-52](./src/eval.rs#L23-L52)):

1. Creates an `EvalContext` struct holding the workspace root `PathBuf` ([eval.rs:17-20](./src/eval.rs#L17-L20))
2. Builds a `GlobalsBuilder` with the custom `codebase_builtins` module ([eval.rs:29-31](./src/eval.rs#L29-L31))
3. Attaches the `EvalContext` to `eval.extra` so native functions can access the sandboxed root ([eval.rs:35](./src/eval.rs#L35))
4. Parses and executes the script with the configured dialect ([eval.rs:38-49](./src/eval.rs#L38-L49))
5. Returns the last expression: as a plain string if it unpack as `str`, otherwise as `repr()` ([eval.rs:51-54](./src/eval.rs#L51-L54))

The dialect is configured as ([eval.rs:37-41](./src/eval.rs#L37-L41)):
```rust
let dialect = Dialect {
    enable_top_level_stmt: true,   // allows bare expressions at top level (return value)
    enable_f_strings: true,        // enables f"..." syntax
    ..Dialect::Standard            // all other Starlark Standard settings
};
```

---

## Built-in Functions ([eval.rs:62-168](./src/eval.rs#L62-L168))

All file I/O functions route through the same `codebase::*` functions used by standalone tools, scoped to the workspace root via `EvalContext`:

**File I/O:**

| Starlark function | Backed by | Notes |
|---|---|---|
| `read_file(path)` | [codebase::read_file](./src/codebase.rs#L179) | Returns full file content as string |
| `read_file_lines(path, start, end)` | [codebase::read_file_lines](./src/codebase.rs#L318) | 1-indexed, inclusive; returns lines with line numbers |
| `find_files(pattern)` | [codebase::find_files](./src/codebase.rs#L490) | Returns `list[str]` of relative paths (filters summary line from raw output) |
| `list_dir(path)` | [codebase::list_directory](./src/codebase.rs#L530) | Returns formatted string with dirs having trailing `/` |
| `grep(pattern, glob="", max_results=50)` | [codebase::grep](./src/codebase.rs#L348) | `context_lines` is hardcoded to `0`; returns formatted `path:line: text` string |

**Text processing:**

| Starlark function | Returns | Notes |
|---|---|---|
| `lines(text)` | `list[str]` | Splits on newlines |
| `regex_find(pattern, text)` | `list[str]` | All non-overlapping matches |
| `regex_match(pattern, text)` | `bool` | True if pattern matches anywhere |

**Data parsing:**

| Starlark function | Returns | Notes |
|---|---|---|
| `json_parse(text)` | `dict\|list\|str\|int\|bool\|None` | Full JSON type mapping via `json_to_starlark()` ([eval.rs:171-198](./src/eval.rs#L171-L198)) |
| `toml_parse(text)` | `dict\|list\|str\|int\|bool` | Full TOML type mapping via `toml_to_starlark()` ([eval.rs:201-225](./src/eval.rs#L201-L225)); `Datetime` values are converted to strings |

---

## Differences from Other AI Tools

### vs. Copilot's Built-in Single-Operation Tools

The built-in Copilot tools (`view`, `grep`, `glob`, etc.) each perform one operation per call. The analyze tool's key advantage is the ability to compose multiple operations with control flow in one round-trip:

| Aspect | Built-in tools | `analyze` |
|---|---|---|
| Operations per call | One | Unlimited (full script) |
| API calls needed | One per operation | One for entire script |
| Data manipulation | None — raw output | Filter, parse, aggregate, format |
| Control flow | None | `for`, `if/elif/else`, `def`, list comprehensions |
| Structured parsing | None | `json_parse`, `toml_parse` |
| Use case | Simple read or search | Cross-referencing, config parsing, aggregation |

### vs. Write Tools

faqifai excludes Copilot's write-capable tools ([ai.rs:315-324](./src/ai.rs#L315-L324)): `edit_file`, `create_file`, `delete_file`. The analyze tool has no write operations at all — `codebase_builtins` only registers read functions ([eval.rs:62-168](./src/eval.rs#L62-L168)).

---

## Limitations Compared to Python

Starlark is intentionally restricted. The base `Dialect::Standard` plus faqifai's overrides (`enable_f_strings`, `enable_top_level_stmt`) give:

### Missing Language Features

| Feature | Python | Starlark |
|---|---|---|
| `import` / modules | ✅ | ❌ |
| `class` | ✅ | ❌ |
| `try` / `except` / `raise` | ✅ | ❌ (errors propagate immediately) |
| `while` loops | ✅ | ❌ — use `for x in range(n)` |
| `set` type | ✅ | ❌ — only `list` and `dict` |
| `yield` / generators | ✅ | ❌ |
| `with` / context managers | ✅ | ❌ |
| `global` / `nonlocal` | ✅ | ❌ |

### Restricted Features

**f-strings** — only simple variable names work in `{}`, not expressions:
```starlark
name = "world"
f"hello {name}"          # ✅ works
f"result: {1 + 2}"       # ❌ fails — no expressions in braces
f"count: {len(items)}"   # ❌ fails — no function calls in braces
```
Use concatenation with `str()` for expressions: `"count: " + str(len(items))`

**No mutable default arguments**: Function defaults are evaluated once.

**No `None`-safe chaining** (`x?.y` syntax doesn't exist).

**Integer overflow**: `json_to_starlark` maps `i64` values to `i32` ([eval.rs:183](./src/eval.rs#L183)), so very large JSON integers are truncated.

### What Does Work

- `for` loops, `if`/`elif`/`else`, `def` (functions), list/dict comprehensions
- All standard builtins: `len`, `range`, `enumerate`, `zip`, `sorted`, `reversed`, `str`, `int`, `float`, `bool`, `list`, `dict`, `type`, `hasattr`, `getattr`, `repr`, `min`, `max`, `any`, `all`
- String methods: `.split()`, `.strip()`, `.startswith()`, `.endswith()`, `.replace()`, `.upper()`, `.lower()`, `.find()`, `.count()`, `.join()`, `.format()`
- List methods: `.append()`, `.extend()`, `.insert()`, `.pop()`, `.remove()`, `.index()`
- Dict methods: `.get(key, default)`, `.keys()`, `.values()`, `.items()`, `.pop()`, `.update()`

# How are multiple questions answered concurrently, and what controls the parallelism?

## Concurrency model: worker pool over output-file groups

The entry point is `answer_questions_concurrent()` ([ai.rs:646-735](./src/ai.rs#L646-L735)), called from the orchestrator ([orchestrator.rs:167](./src/orchestrator.rs#L167)) after all stale questions are collected.

---

## Step 1: Grouping by output file

Before spawning any workers, questions are **grouped by their output file path** ([ai.rs:663-672](./src/ai.rs#L663-L672)):

```rust
let mut groups: HashMap<PathBuf, Vec<(usize, QuestionInput)>> = HashMap::new();
for (i, q) in questions.into_iter().enumerate() {
    groups.entry(q.output_path.clone()).or_default().push((i, q));
}
// Sort each group by original index so questions are answered in declaration order
for group in groups.values_mut() {
    group.sort_by_key(|(i, _)| *i);
}
```

Questions that write to the **same output file are answered sequentially in a single Copilot session** — this allows the AI to reuse loaded file context across questions in that group, avoiding redundant reads. Questions targeting **different output files** run in separate, concurrent sessions.

---

## Step 2: Single shared Copilot client

A **single `Client`** (one spawned `copilot --headless --stdio` process) is created and shared across all workers via `Arc<Client>` ([ai.rs:652-658](./src/ai.rs#L652-L658)):

```rust
let client = Arc::new(
    Client::new(ClientOptions {
        working_directory: Some(root.to_path_buf()),
        ..Default::default()
    })
    .await?,
);
```

Multiple sessions can multiplex over this one process concurrently.

---

## Step 3: Worker pool

A fixed-size worker pool is spawned with `tokio::spawn` ([ai.rs:681-720](./src/ai.rs#L681-L720)):

```rust
let workers = concurrency.min(num_groups);   // never more workers than groups
```

Each worker loops until the shared queue is empty, pulling one group at a time:

```rust
loop {
    let group = { queue.lock().await.pop() };
    let group = match group { Some(g) => g, None => break };
    // answer the whole group sequentially in one session
    let group_results =
        answer_questions_in_session(&root, &inputs, &client, &model).await;
    // store results by original index
}
```

The queue is an `Arc<Mutex<Vec<...>>>` shared across all workers. Workers steal from it greedily — if one finishes its group early, it immediately takes the next available group.

---

## Step 4: Sequential session loop (`answer_questions_in_session`)

Within a group, all questions are answered **one at a time in a single `Session`** ([ai.rs:544-640](./src/ai.rs#L544-L640)):

1. One `Session` is created for the group's output file with the shared `Client`
2. `SessionSharedState` tracks per-question state: `current_question`, `unchanged_set`, `recorded_sources` ([ai.rs:22-49](./src/ai.rs#L22-L49))
3. For each question in declaration order:
   - `shared.set_current(question)` — sets current question and **clears the recorded_sources buffer** ([ai.rs:37-40](./src/ai.rs#L37-L40))
   - `session.send_and_wait()` — sends the user message and blocks until the AI finishes
   - `shared.drain_sources()` — collects all `record_source` calls made during that question
4. After all questions, the session is destroyed ([ai.rs:635](./src/ai.rs#L635))

---

## Step 5: Result ordering

Each result is stored with its original question index ([ai.rs:714-717](./src/ai.rs#L714-L717)). After all workers finish, results are **sorted back to the original declaration order** before returning ([ai.rs:729-734](./src/ai.rs#L729-L734)):

```rust
results.sort_by_key(|(idx, _)| *idx);
Ok(results.into_iter().map(|(_, r)| r).collect())
```

---

## Controlling parallelism

### CLI flag

```
--concurrency <N>    Number of questions to answer in parallel [default: 4]
```

Defined in [main.rs](./src/main.rs) with `default_value_t = 4`, passed through `orchestrator::run()` → `ai::answer_questions_concurrent()`.

### Effective parallelism cap

The actual number of concurrent workers is `min(concurrency, num_groups)` ([ai.rs:683](./src/ai.rs#L683)). If only 2 output files exist, at most 2 sessions run simultaneously regardless of `--concurrency`.

### Summary

| Level | Parallelism |
|---|---|
| Questions sharing one output file | **Sequential** (same session, shared AI context) |
| Questions targeting different output files | **Concurrent** (separate sessions, up to `--concurrency` at once) |
| Copilot CLI processes | **One** (shared `Arc<Client>`) |
| Worker tasks | `min(--concurrency, number_of_output_files)` |

# What prompts are given to Copilot for generating answers?

## System Prompt

The core system prompt is the `SYSTEM_PROMPT` constant at [ai.rs:54-161](./src/ai.rs#L54-L161). It instructs the AI to act as a "codebase research agent generating reference documentation that will be consumed by other AI agents and developers."

### Directives

1. **Be thorough** — Trace call chains, follow imports, read config files; include edge cases and error paths
2. **Be factual and unopinionated** — State what the code does, not what it should do; never speculate
3. **Reference source files** — Use markdown links with relative paths; link text must include the filename; explicit WRONG/RIGHT examples are provided
4. **Use tools aggressively** — grep, glob, view files; use `analyze` for multi-step analysis instead of ≥3 sequential calls; build on prior research within a multi-question session
5. **Structure for scanability** — Headings, bullets, code blocks; lead with the direct answer
6. **Output only the answer body** — No preamble, meta-commentary (`"Perfect!"`, `"Let me create..."`), or sign-off; first character must be content
7. **Handle previous answers** — Treat as a research accelerator; verify all claims; call `mark_unchanged` (no output) if nothing has changed
8. **Track sources with `record_source`** — Call after reading every materially influential file; supports whole-file, directory (`trailing/`), and specific line ranges (`start_line`+`end_line`); staleness checking follows moved content

The prompt also includes a **complete Starlark scripting reference** for the `analyze` tool: function signatures, language restrictions vs Python, and three worked examples (cross-referencing, TOML parsing, JSON parsing).

### Path resolution ([ai.rs:176-189](./src/ai.rs#L176-L189))

`SYSTEM_PROMPT` contains a `{output_dir}` placeholder. `build_system_prompt()` replaces it at runtime using `relative_prefix_to_root()`, which counts the depth of the output file's parent directory:

| Output path | `{output_dir}` becomes |
|---|---|
| `faq.md` | `.` |
| `docs/faq.md` | `..` |
| `docs/api/faq.md` | `../..` |

This lets the AI produce correct relative file links (e.g., `[main.rs](../../src/main.rs)`) for output files nested anywhere in the repo.

The system message is sent with `"mode": "replace"` ([ai.rs:362](./src/ai.rs#L362)), overriding any default Copilot system prompt.

---

## User Message

Built by `build_user_message()` ([ai.rs:193-255](./src/ai.rs#L193-L255)) from up to five optional parts, assembled in this order:

### 1. Context *(optional)*

```markdown
## Context

{context text from .faq file}
```

Included when the `.faq` file specifies a `context` field for the question.

### 2. Suggested Starting Points *(optional)*

```markdown
## Suggested Starting Points

- `src/path/to/file.rs`
- `src/other/file.rs`
```

Populated from the question's `hints` field in the `.faq` file. Omitted if the list is empty.

### 3. Previous Answer *(regeneration only)*

When a stale answer exists, three sub-sections are included ([ai.rs:220-248](./src/ai.rs#L220-L248)):

```markdown
## Previous Answer (for reference only)

The following is a previous answer to this question. It may be partially
or fully outdated. Use it to guide your research and help cross-check what
is still correct, but do NOT trust it — verify every claim against the
current source code.

**Detected changes since last generation:**
- `src/changed_file.rs`
- `src/changed_dir/`

Pay special attention to these changed sources — they are likely why the
answer needs updating.

**Files previously tracked as relevant** (call `record_source` for each
that remains relevant — add new findings, omit any that no longer apply):
- `src/previously/tracked.rs`

<previous_answer>
{the old answer text}
</previous_answer>
```

- The `changed_sources` list only appears if `Staleness::SourcesChanged` — it's absent for `TtlExpired` or `NeverGenerated`
- The `previous_sources` list only appears if a previous answer exists and had tracked sources

### 4. Question *(always present)*

```markdown
## Question

{question text from .faq file}
```

---

## Custom Tool Definitions ([ai.rs:258-312](./src/ai.rs#L258-L312))

Three custom tools are registered, in addition to the Copilot CLI's built-in read tools:

### `analyze`
Runs a Starlark script for multi-step codebase analysis. Parameters: `intent` (one-line description) and `script` (the Starlark code). Returns the last expression as a string.

### `mark_unchanged`
Signals that the previous answer is still fully accurate. Takes no parameters. When called, it inserts the question into an `unchanged_set`; the orchestrator then reuses the old answer verbatim instead of the (potentially empty) AI response.

### `record_source`
Records a file as relevant to the current answer for staleness tracking. Parameters:
- `path` (required) — relative path; trailing `/` for directories
- `start_line` / `end_line` (optional) — track only a specific line range; staleness detection will follow the content even if it moves

---

## Excluded Tools ([ai.rs:315-324](./src/ai.rs#L315-L324))

Three built-in Copilot tools are explicitly blocked to enforce read-only access:
- `edit_file`
- `create_file`
- `delete_file`

---

## Session Configuration ([ai.rs:358-370](./src/ai.rs#L358-L370))

Each session is created with:

```rust
SessionConfig {
    model: Some(model),               // from --model CLI flag
    system_message: Some(SystemMessageConfig {
        mode: "replace",              // overrides Copilot's default system prompt
        content: system_prompt,       // SYSTEM_PROMPT with {output_dir} resolved
    }),
    tools: Some(tool_definitions()),  // 3 custom tools: analyze, mark_unchanged, record_source
    excluded_tools: Some(excluded_tools()), // 3 blocked: edit_file, create_file, delete_file
    working_directory: Some(root),    // workspace root (repo CWD)
}
```

# how does faqifai optimize token and request usage?

## Source-hash staleness: skip regeneration entirely

Each generated answer records which files the AI read, together with their SHA-256 hashes, in the output file's YAML frontmatter ([state.rs:36-44](./src/state.rs#L36-L44)). On every subsequent run, `check_staleness()` ([state.rs:112-179](./src/state.rs#L112-L179)) recomputes hashes and only sends the question to the AI when:

- The output entry doesn't exist yet → `NeverGenerated`
- The configured TTL has elapsed → `TtlExpired`
- A tracked source hash differs → `SourcesChanged`

Questions that pass all checks return `Fresh` and are never sent to the AI. The orchestrator simply carries forward their existing text ([orchestrator.rs:98-100](./src/orchestrator.rs#L98-L100), [orchestrator.rs:202-209](./src/orchestrator.rs#L202-L209)).

---

## Line-range source tracking: surgical staleness

When the AI calls `record_source` with `start_line`+`end_line`, only that block of lines is hashed ([state.rs:27-34](./src/state.rs#L27-L34), [codebase.rs:239-249](./src/codebase.rs#L239-L249)). Staleness checking then uses `check_line_range_staleness()` ([codebase.rs:251-315](./src/codebase.rs#L251-L315)) with **content-following logic**: if the hash doesn't match at the original position, the function scans every same-sized window in the file filtered by byte-length before hashing. This means adding unrelated lines above a tracked function does not trigger regeneration — only an actual change to the tracked content does.

---

## Hash cache: one hash per file per run

A single `HashMap<String, String>` is allocated at the start of `orchestrator::run()` ([orchestrator.rs:24](./src/orchestrator.rs#L24)) and passed into every `check_staleness()` and `collect_context()` call. `hash_source_cached()` ([codebase.rs:209-234](./src/codebase.rs#L209-L234)) checks the cache before doing any I/O, so a file referenced by ten questions is only read and hashed once per run.

---

## Merkle hashing: one hash per directory/glob

Tracking `src/**` as a scope doesn't store a hash per file. Instead, `walk_glob` / `walk_dir` collect all matching `(rel_path, sha256)` pairs and feed them into `merkle_hash()` ([codebase.rs:34-48](./src/codebase.rs#L34-L48)) — a single SHA-256 over all sorted entries. The frontmatter stores one hash regardless of how many files are in the scope. Additions, deletions, and content changes all alter the merkle hash.

---

## Parallel file hashing with Rayon

When walking directories or globs for merkle computation, content hashing runs in parallel via `rayon::into_par_iter()` ([codebase.rs:81-87](./src/codebase.rs#L81-L87), [codebase.rs:114-120](./src/codebase.rs#L114-L120)). Path discovery is sequential (the `ignore` walker), but the read + SHA-256 step for every discovered file runs across all CPU cores.

---

## Session sharing within an output group

Questions targeting the **same output file** are answered sequentially inside **one Copilot session** ([ai.rs:541-543](./src/ai.rs#L541-L543)):

> "The session retains file context across questions — loaded files stay in context."

Files the AI reads for question 1 stay in the session's context window for question 2, 3, etc. This avoids redundant file reads across related questions and saves both input tokens and round-trips.

---

## Shared Copilot process across all workers

All concurrent sessions share one `Arc<Client>` ([ai.rs:652-658](./src/ai.rs#L652-L658)) — a single spawned `copilot --headless --stdio` process. Sessions are lightweight multiplexed conversations over this one process rather than separate subprocesses.

---

## `mark_unchanged`: skip generation when verified fresh

When re-checking a stale answer, the AI receives the previous answer and the list of changed sources. If after investigation it finds nothing actually changed, it calls the `mark_unchanged` tool ([ai.rs:435-442](./src/ai.rs#L435-L442)). The orchestrator then reuses the existing answer verbatim, preserving the original `generated_at` timestamp and spending zero output tokens.

---

## Previous answer as a differential-update accelerator

When regenerating, `build_user_message()` ([ai.rs:220-248](./src/ai.rs#L220-L248)) includes the old answer text and the specific list of changed paths. The AI can reuse unchanged sections verbatim and restrict investigation to the flagged files, rather than researching the entire codebase from scratch.

---

## `analyze` tool: batch reads in one call

The AI is instructed to use the `analyze` Starlark tool instead of chaining ≥3 sequential read/grep calls ([ai.rs:76](./src/ai.rs#L76)). A single `analyze` call that reads 10 files, parses TOML, and cross-references grep results counts as one tool-call round-trip ([eval.rs:62-168](./src/eval.rs#L62-L168)).

---

## Incremental output merge: only rewrite what changed

Fresh answers are preserved exactly from the existing output file ([orchestrator.rs:202-209](./src/orchestrator.rs#L202-L209)). Only newly regenerated answers replace their entries. The orchestrator merges the two sets and writes one output file per group ([orchestrator.rs:219-226](./src/orchestrator.rs#L219-L226)).

---

## No file content in prompts

The user message contains only context text, hint paths (not content), and optionally the previous answer — never raw file content ([ai.rs:193-255](./src/ai.rs#L193-L255)). The AI fetches files itself on demand via Copilot's built-in tools, so the prompt size is constant regardless of codebase size.

---

## Compact tool-call logging (noise reduction, not token saving)

`session.rs` suppresses and abbreviates tool call output to keep stderr readable:
- `mark_unchanged` and `report_intent` calls are not logged ([session.rs:41-42](./src/copilot/session.rs#L41-L42), [session.rs:254](./src/copilot/session.rs#L254))
- For `analyze`, only the `intent` argument is printed (not the full script) ([session.rs:43-45](./src/copilot/session.rs#L43-L45))
- Shell commands are truncated to 80 characters ([session.rs:522](./src/copilot/session.rs#L522))
- Paths inside temp directories (`/tmp/`, `/var/folders/`, `AppData\Local\Temp`) are suppressed ([session.rs:492-493](./src/copilot/session.rs#L492-L493))
- Absolute paths inside the workspace are relativized ([session.rs:488-501](./src/copilot/session.rs#L488-L501))

---

## TTL: user-controlled freshness vs. cost tradeoff

Questions can specify `ttl = "7d"` or similar in the `.faq` file. Stable documentation questions can use long or no TTL, avoiding any regeneration cost until sources actually change. Fast-moving questions use short TTLs. Without a TTL, only source hash changes trigger regeneration.

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