# Security and quality

## Reporting

Use private vulnerability reporting — see [SECURITY.md](https://github.com/ao3575911/glossa/blob/main/SECURITY.md). Never file public issues for exploitable defects.

## Automated gates

| Workflow | Purpose |
| --- | --- |
| `CI` | fmt, clippy `-D warnings`, tests (Ubuntu / macOS / Windows), bench smoke |
| `Security and quality` | cargo-deny, cargo-audit, CodeQL |
| `Labels` | Sync `.github/labels.yml` |
| `PR labeler` | Path → `area:*` / `type:*` labels |
| `Pages` | Deploy `docs/pages` + wiki mirror |
| `Stale` | Nudge inactive issues/PRs |

Policy files: `deny.toml`, `.github/codeql/codeql-config.yml`, `rustfmt.toml`, `clippy.toml`.

## Local checks

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
cargo audit
```
