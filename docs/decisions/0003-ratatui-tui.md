# ADR 0003: In-process Ratatui TUI

## Status

Accepted (Stage 4 MVP)

## Context

Glossa’s build spec ([`docs/07-build-spec.md`](../07-build-spec.md)) calls for a **ratatui + crossterm** TUI in `crates/glossa-tui`, sharing logic with the CLI via `glossa-core` / workspace crates—not a separate Python Textual app driving `glossa --json` subprocesses.

## Decision

1. Add **`glossa-commands`** for init, corpus, and KWIC logic used by both CLI and TUI.
2. Implement the MVP TUI in **`glossa-tui`** (tabs: workspace, concordance, stats stub, session log).
3. Launch with bare **`glossa`** (interactive TTY) or **`glossa tui`**; set **`GLOSSA_NO_TUI=1`** to keep banner-only behaviour for scripts.
4. Defer a Python Textual UI; reuse its UX checklist only as product input.

## Consequences

- Single binary / install story; no second runtime or JSON bridge drift.
- Long imports run on a background thread; UI stays responsive via polling.
- Stats tab stays a stub until `glossa freq` / colloc / keyness exist in the CLI.
- TUI tests use ratatui `TestBackend` + insta buffer snapshots at 80×24.
