# Contributing to Glossa

Thanks for helping build an offline-first linguistics workspace. This guide covers
issues, Discussions, pull requests, and local development.

## Before you start

1. Read [docs/06-PRD.md](docs/06-PRD.md) and [docs/07-build-spec.md](docs/07-build-spec.md).
2. Skim [AGENTS.md](AGENTS.md) if you use coding agents.
3. Search [Issues](https://github.com/ao3575911/glossa/issues) and
   [Discussions](https://github.com/ao3575911/glossa/discussions) for duplicates.

## Ways to contribute

| Path | Use when |
| --- | --- |
| Bug / feature / docs / performance **issue templates** | Actionable work with labels |
| **Discussions** (Ideas, Q&A, Show and tell) | Open-ended questions and proposals |
| **Pull request** | Ready-to-review code or docs |
| **Wiki** (`docs/wiki` mirrored + GitHub Wiki) | Longer-form guides |
| **Security advisory** | Vulnerabilities — see [SECURITY.md](SECURITY.md) |

Newcomers: filter issues by [`good first issue`](https://github.com/ao3575911/glossa/labels/good%20first%20issue).

## Labels

Canonical labels live in [`.github/labels.yml`](.github/labels.yml):

- `type:*` — bug, feature, docs, chore, …
- `area:*` — crate or concern (`cli`, `index`, `nlp`, …)
- `priority:*` / `status:*` / `size:*` — planning
- `pr:*` — pull-request markers (`breaking`, `dependencies`, …)

PR paths are auto-labelled by [`.github/labeler.yml`](.github/labeler.yml).

## Development

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo bench -p glossa-index --bench index_kwic
```

Rules of thumb:

- No `unwrap()` in library crates; prefer `thiserror` in libs and `anyhow` in the binary.
- Keep output ordering deterministic.
- Record intentional spec deviations in `docs/decisions/NNNN-title.md`.
- Do not commit secrets, private corpora, or large generated artefacts.

## Pull requests

1. Branch from `main` with a focused scope.
2. Fill out the PR template and link issues (`Fixes #N`).
3. Ensure CI is green (fmt, clippy, tests on Ubuntu / macOS / Windows).
4. Request review when labelled ready (`pr:ready`); use draft PRs while iterating.

## Code of conduct

Participation is governed by [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

## License

By contributing, you agree that your contributions are licensed under the MIT
License (see [LICENSE](LICENSE)).
