# Recommended next prompt

Paste this into a coding agent with repo access:

```
ROLE
You are a senior Rust engineer building Glossa, an offline-first linguistics CLI/TUI (command: `glossa`). Read docs/06-PRD.md and docs/07-build-spec.md in this repo first; they are the source of truth.

TASK
Implement Stage 0 (Foundations) and Stage 1 (Import + index + KWIC) exactly as specified in docs/07-build-spec.md section 11.

STAGE 0
- Cargo workspace with crates: glossa-cli, glossa-core, glossa-io, glossa-index, glossa-workspace (others as empty stubs: glossa-tui, glossa-nlp, glossa-stats, glossa-plugin).
- clap-derive CLI with global flags --json, --plain, --no-color, --workspace.
- A result envelope in glossa-core: { glossa_version, command, params, corpus_hash, model_hashes, started_at, duration_ms, data } used by every command's --json output.
- `glossa init [dir]` creates glossa.toml and .glossa/ with SQLite (rusqlite bundled) and session.jsonl; every command appends one log line.
- GitHub Actions CI: fmt, clippy -D warnings, test on ubuntu/macos/windows. cargo-dist config for releases.

STAGE 1
- Importers: TXT (encoding detection via chardetng + encoding_rs), HTML (strip to text), DOCX (zip + quick-xml), PDF text layer, CoNLL-U (keep annotations). Optional metadata CSV keyed by filename.
- Rule-based tokeniser and sentence splitter using UAX #29 (unicode-segmentation) plus an English abbreviation list; NFC normalisation.
- Positional index: per-attribute fst vocabulary, u32 token-id columns via memmap2, delta+varint postings, sentence/document boundaries, SHA-256 corpus hash.
- `glossa corpus add|ls|info|rm` and `glossa kwic <query> [--regex] [--sort L1|R1|R2] [--context N] [--limit N]` with aligned plain output and JSON output.

TESTS / DONE WHEN
- Unit + proptest for tokeniser and index round-trip; insta golden tests for corpus add and kwic JSON on a fixture corpus in tests/fixtures (use public-domain Gutenberg text).
- A criterion benchmark: indexing a 10M-token synthetic corpus takes < 30 s and a kwic query < 1 s on CI-class hardware (report numbers).
- CI green on all three OS.

RULES
- No unwrap() in library code; use thiserror in libs, anyhow in the binary.
- Deterministic output ordering.
- Keep commits small and conventional; update README with real usage examples when done.
- If a spec decision proves wrong, change the code, write the reason in docs/decisions/NNNN-title.md, and continue; do not stop to ask.
Finish by reporting: what was built, benchmark numbers, anything deferred.
```
