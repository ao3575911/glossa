# Agent guide — Glossa

Instructions for coding agents and humans working in this repository.

## Product

**Glossa** is an offline-first linguistics CLI/TUI (`glossa`). Current scope is
Stage 0–1: workspaces, corpus import, positional index, and KWIC. Brand direction
is manuscript-terminal (see `brand/tokens.json`).

## Source of truth

| Document | Role |
| --- | --- |
| `docs/06-PRD.md` | Product requirements |
| `docs/07-build-spec.md` | Technical architecture and stages |
| `docs/decisions/` | Accepted deviations (ADRs) |
| `SECURITY.md` | Vulnerability reporting |
| `.github/labels.yml` | Issue/PR label taxonomy |

Read the PRD and build spec before implementing behaviour changes.

## Repository map

```
crates/glossa-cli         # binary
crates/glossa-core        # result envelope / output mode
crates/glossa-workspace   # init, SQLite, session.jsonl
crates/glossa-io          # importers
crates/glossa-nlp         # UAX #29 tokeniser + sentence split
crates/glossa-index       # positional index + KWIC
crates/glossa-tui|stats|plugin  # stubs for later stages
brand/                    # icon, logo, social, terminal banners
docs/                     # product docs + wiki source + pages
.github/                  # Actions, templates, labels, seeds
```

## Engineering rules

- Rust 2021, workspace edition; MSRV declared in `Cargo.toml` (`rust-version`).
- No `unwrap()` / `expect()` in library code paths; `thiserror` in libs, `anyhow` in the CLI.
- Deterministic output ordering for CLI and JSON envelopes.
- Prefer small, conventional commits; keep PRs reviewable.
- When the spec is wrong, change the code **and** add `docs/decisions/NNNN-title.md`.
- Never commit secrets, credentials, or private corpus material.
- Do not expand scope into later stages unless the task explicitly asks.

## Verification

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Optional: `cargo bench -p glossa-index --bench index_kwic` (non-Windows CI smoke).
Security/quality: `cargo deny check`, `cargo audit` (see `.github/workflows/security.yml`).

## Issues, Discussions, PRs

- Use issue templates; labels are applied from the taxonomy.
- Prefer Discussions for questions; convert to issues when actionable.
- Fill the PR template; path-based labels come from `.github/labeler.yml`.
- Seed labelled starter issues via **Actions → Seed labelled issues** (disable dry-run).

## Wiki and Pages

- Editable long-form docs live in `docs/wiki/` (mirrored to GitHub Wiki / Pages).
- Static site entrypoint: `docs/pages/` deployed by `.github/workflows/pages.yml`.

## Out of scope for agents unless asked

- Rewriting brand identity or inventing new visual systems
- Enabling org-level GitHub billing features
- Publishing crates.io releases without an explicit release task
