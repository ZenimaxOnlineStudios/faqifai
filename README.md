# faqifai

AI-powered FAQ generator for codebases. Write questions in `.faq` files, and faqifai uses GitHub Copilot to research your codebase and produce terse, sourced markdown answers — optimized for consumption by other AI agents and developers.

## How it works

1. You create `.faq` files (TOML) anywhere in your repository with questions about the codebase
2. `faqifai run` discovers them, checks which answers are stale or missing, and spawns AI sessions to answer them
3. Each session uses Copilot's built-in tools (file reading, search, grep) plus a sandboxed Starlark scripting tool (`analyze`) for multi-step analysis in a single call
4. Answers are written to markdown files with a human-readable table of contents, YAML frontmatter, and relative file links
5. The AI reports which files it used — only those specific files are hashed for change detection, so reruns are cheap
6. On subsequent runs, only stale answers are regenerated — controlled by TTL expiry and source file change detection

## Installation

Requires [GitHub Copilot CLI](https://docs.github.com/en/copilot) installed and authenticated.

```sh
cargo install --path .
```

## Quick start

Create a `.faq` file in your repository root:

```toml
ttl = "7d"
output = "faq.md"
scope = "src/**"
context = "This is a Rust web server using Axum."

[[question]]
text = "How does authentication work?"
hints = ["src/auth/mod.rs"]

[[question]]
text = "What database migrations exist and how are they run?"
hints = ["migrations/"]
```

Then run:

```sh
faqifai run
```

All paths in `.faq` files are relative to the `.faq` file's location. The codebase root is the directory where you invoke `faqifai`.

## FAQ file format

`.faq` files use TOML with top-level defaults and per-question overrides:

| Field     | Level          | Description                                          |
|-----------|----------------|------------------------------------------------------|
| `output`  | top / question | Path to output markdown file (required)              |
| `ttl`     | top / question | Regeneration interval (`7d`, `24h`, `30m`)           |
| `scope`   | top / question | Glob pattern for source files to watch for changes   |
| `context` | top / question | Free-text context given to the AI                    |
| `hints`   | question only  | File paths the AI should prioritize reading          |

Per-question fields override top-level defaults. Context fields are concatenated.

## Commands

```
faqifai run [--force] [--path DIR] [--concurrency N] [--model MODEL]
```
Generate or regenerate FAQ answers. `--force` ignores TTL and hash state. Default concurrency is 4 parallel sessions. Default model is `claude-sonnet-4.6`.

```
faqifai status [--path DIR]
```
Show staleness status of all questions (fresh / TTL expired / sources changed / never generated).

```
faqifai list [--path DIR] [--format text|json]
```
List all known questions and their output files.

```
faqifai get <query> [--path DIR] [--raw]
```
Retrieve the answer to a specific question by substring match. `--raw` prints just the answer body.

```
faqifai search <pattern> [--path DIR] [--format text|json]
```
Search across all generated answers for a keyword or pattern.

## Staleness detection

An answer is regenerated when any of these conditions is met:

- **Never generated** — question exists in `.faq` but no answer in the output file
- **TTL expired** — the `generated_at` timestamp plus the TTL has passed
- **Sources changed** — the AI-reported source files have changed since last generation (SHA-256 / merkle hashes)
- **Force flag** — `--force` regenerates everything unconditionally

When regenerating, the previous answer and the list of previously-tracked source files are passed to the AI so it can update efficiently rather than starting from scratch.

The staleness reason is printed before each question runs:
```
⚡ Stale (sources changed: src/auth/mod.rs): How does authentication work?
```

## Output format

Generated markdown files include a human-readable table of contents at the top, followed by answers as `#`-headed sections:

```markdown
## Contents

- [How does authentication work?](#how-does-authentication-work)
- [What database migrations exist?](#what-database-migrations-exist)

---

# How does authentication work?

...answer...
```

The YAML frontmatter (hidden from most markdown viewers) stores per-question generation timestamps and source file hashes for staleness tracking.

File references in answers are relative markdown links pointing back to the source, e.g. `[auth/mod.rs:42](./src/auth/mod.rs#L42)`.

## Analyze tool

In addition to Copilot's built-in file tools, faqifai provides an `analyze` tool that executes sandboxed [Starlark](https://github.com/bazelbuild/starlark) scripts. This lets the AI do multi-step analysis — parse configs, cross-reference symbols, aggregate data — in a single call instead of many sequential reads.

Available built-ins: `read_file`, `read_file_lines`, `find_files`, `list_dir`, `grep`, `lines`, `regex_find`, `regex_match`, `json_parse`, `toml_parse`.

The sandbox is read-only with no network access and no shell execution. All paths are scoped to the workspace root.

## Security

- Write tools (`edit_file`, `create_file`, `delete_file`) are excluded from AI sessions
- `run_command` requires interactive user approval before execution
- The Starlark sandbox is read-only with no network or shell access
- All file operations are scoped to the workspace root via path validation and symlink checks

## Development

Requires Rust 2024 edition. Uses [just](https://github.com/casey/just) as a task runner:

```sh
just build          # Build the project
just test           # Run all tests
just run            # Run against current directory
just status         # Show question staleness
just clean          # Remove generated output files
just regen          # Force regenerate all answers
just fresh          # Clean + regenerate
just run-trace      # Run with trace-level logging
```

## Generated FAQ

See [faqifai.md](faqifai.md) for AI-generated answers about this codebase's internals — produced by faqifai itself.

## License

MIT
