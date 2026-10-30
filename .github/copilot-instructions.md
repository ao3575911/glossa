# Copilot instructions — Glossa

Read `AGENTS.md` first; it is the source of truth for scope, layout and rules.

- Glossa is an offline-first Rust CLI/TUI corpus workbench (ratatui). It is not a model-driven measurement lab; leave LM surprisal and embeddings to langSuite.
- Rust 2021; respect `rust-version` in `Cargo.toml`.
- Libraries: no `unwrap()`/`expect()`; errors via `thiserror`. CLI: `anyhow`.
- Keep CLI and JSON output deterministic; every command returns the provenance envelope from `glossa-core`.
- Behaviour changes follow `docs/06-PRD.md` and `docs/07-build-spec.md`; deviations need an ADR in `docs/decisions/`.
- Conventional commits; small PRs; fill the PR template.
- Before proposing a PR, these must pass:
  `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `cargo deny check`.
- Never add secrets, credentials or private corpus text.
