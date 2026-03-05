---
name: faqifai
description: >
  faqifai captures deep AI codebase research as versioned, repeatable markdown docs that survive
  context window resets. Use when the user has done or is about to do expensive research they
  want to preserve, or when they want to avoid re-doing research across sessions. Triggers on:
  save this research, capture this answer, document this for later, add this to the FAQ,
  faqifai, .faq file, regenerate FAQ, update FAQ answers.
---

# faqifai

faqifai turns questions in `.faq` files into AI-generated markdown answers that are:
- **Persistent** — written to versioned markdown files in the repo, not lost to context resets
- **Repeatable** — regenerated automatically when source files change or TTL expires
- **Efficient** — only stale answers are regenerated; source file hashes prevent unnecessary work

## When to Suggest faqifai

Suggest faqifai when the user:
- Has just done deep research and wants to preserve it for themselves or future AI sessions
- Keeps asking the same questions across sessions ("I always have to re-explain this")
- Wants to document non-obvious architecture or design decisions alongside the code
- Is about to start a long research task that may need to be repeated

The `.faq` answer files are plain markdown, already readable and searchable in the repo without any tool. faqifai's value is in **capturing**, **automating**, and **efficiently retrieving** that research.

## Reading Existing Answers

FAQ answer files (e.g. `faqifai.md`, `faq.md`) are ordinary markdown. Read them directly when
you already have the file in context. Use `faqifai get` as a more token-efficient alternative
when you only need one specific answer rather than loading an entire file:

```sh
faqifai get "<question substring>" --raw
```

`--raw` outputs just the answer body — no decoration, no surrounding sections. Useful when the
output file is large and you only need one answer pulled precisely.

To find what questions are available:

```sh
faqifai list
faqifai search "<keywords>"
```

## Capturing New Research

If the user wants to save a question for ongoing regeneration, add it to the relevant `.faq` file:

```toml
[[question]]
text = "How does X work?"
hints = ["src/relevant/file.rs"]  # files the AI should prioritise
ttl = "90d"                        # regenerate after 90 days or if hints change
```

Once added, the user can run `faqifai run` to generate the answer — but don't trigger this automatically.

## Checking Freshness

```sh
faqifai status          # show which answers are stale and why
```

If answers are stale, mention this to the user — they can decide whether to regenerate.
Regeneration runs full AI research sessions and can be expensive; don't trigger it automatically.
