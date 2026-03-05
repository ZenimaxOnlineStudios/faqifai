# faqifai

AI-powered FAQ generator for codebases. Write questions in `.faq` files, and faqifai uses GitHub Copilot to research your codebase and produce terse, sourced markdown answers — optimized for consumption by other AI agents and developers.

## How it works

1. You create `.faq` files (TOML) anywhere in your repository with questions about the codebase
2. `faqifai run` discovers them, checks which answers are stale or missing, and spawns AI sessions to answer them
3. Each session uses Copilot's built-in tools (file reading, search, grep) plus a sandboxed Starlark scripting engine to thoroughly research the codebase
4. Answers are written to markdown files with YAML frontmatter tracking source file hashes and generation timestamps
5. On subsequent runs, only stale answers are regenerated — controlled by TTL expiry and source file change detection

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

## FAQ file format

`.faq` files use TOML with top-level defaults and per-question overrides:

| Field     | Level        | Description                                          |
|-----------|--------------|------------------------------------------------------|
| `output`  | top / question | Path to output markdown file (required)            |
| `ttl`     | top / question | Regeneration interval (`7d`, `24h`, `30m`)         |
| `scope`   | top / question | Glob pattern for source files to include as context |
| `context` | top / question | Free-text context given to the AI                  |
| `hints`   | question only  | File paths the AI should prioritize reading        |

Per-question fields override top-level defaults. Context fields are concatenated.

## Commands

```
faqifai run [--force] [--path DIR] [--concurrency N]
```
Generate or regenerate FAQ answers. `--force` ignores TTL and hash state. Default concurrency is 4 parallel sessions.

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
- **Sources changed** — SHA-256 hashes of source files tracked in the YAML frontmatter differ from current disk state
- **Force flag** — `--force` regenerates everything unconditionally

## Output format

Generated markdown files have YAML frontmatter with a table of contents, per-question generation timestamps, and source file SHA-256 hashes for change detection:

```yaml
---
generated_by: faqifai
source: project.faq
toc:
- question: How does authentication work?
  anchor: '#how-does-authentication-work'
  generated_at: 2026-03-05T17:04:59Z
  sources:
  - path: src/auth/mod.rs
    sha256: abc123...
---
```

Answer bodies use sub-headings, code blocks, and relative file path links for traceability.

## Starlark eval tool

In addition to Copilot's built-in file tools, faqifai provides an `eval` tool that executes sandboxed [Starlark](https://github.com/bazelbuild/starlark) scripts. This lets the AI do multi-step analysis (parse configs, cross-reference symbols, aggregate data) in a single tool call. The sandbox provides file I/O, regex, and JSON/TOML parsing — but no network access, no shell execution, and no write operations.

## Security

- Write tools (`edit_file`, `create_file`, `delete_file`) are excluded from AI sessions
- `run_command` requires interactive user approval before execution
- The Starlark sandbox is read-only with no network or shell access
- All file operations are scoped to the workspace root

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
