# Glossa

[![CI](https://github.com/ao3575911/glossa/actions/workflows/ci.yml/badge.svg)](https://github.com/ao3575911/glossa/actions/workflows/ci.yml)
[![Security](https://github.com/ao3575911/glossa/actions/workflows/security.yml/badge.svg)](https://github.com/ao3575911/glossa/actions/workflows/security.yml)
![Rust](https://img.shields.io/badge/rust-stable-orange)
[![License: MIT](https://img.shields.io/badge/license-MIT-green)](LICENSE)

An offline corpus workbench for the terminal: import, index and read concordances fast.

![Import and KWIC from the CLI](docs/assets/glossa-cli.gif)

## Install

Pre-release; build from source.

```bash
git clone https://github.com/ao3575911/glossa && cd glossa
cargo install --path crates/glossa-cli
```

## Quickstart

```bash
glossa init study && cd study
glossa corpus add ../alice.txt --name alice     # TXT, HTML, DOCX, PDF text layer, CoNLL-U
glossa kwic Rabbit --corpus alice --sort R1 --context 5 --limit 8
glossa kwic 'Hatt[a-z]+' --regex --corpus alice
```

On *Alice's Adventures in Wonderland* (Project Gutenberg #11), import indexes 37,813 tokens and 1,806 sentences in under 0.1 s.

## Common workflows

**Read concordances in the TUI.** Run `glossa tui` (or bare `glossa`) in a workspace. Press `2` for Concordance, `/` to search, `j`/`k` to move, `s` to cycle sort (L1/R1/R2), `Ctrl+e` to export CSV, `q` to quit.

![Concordance in the TUI](docs/assets/glossa-tui.gif)

**Script it.** Every command takes `--json` and returns a provenance envelope (version, params, corpus hash, timing). `--plain` gives screen-reader-friendly output.

**Attach metadata.** `glossa corpus add DIR --name NAME --meta meta.csv`, then `glossa corpus ls` and `glossa corpus info NAME`.

## Concepts

- **Workspace**: a directory with `glossa.toml` and `.glossa/` (SQLite metadata, corpora, session log).
- **Corpus**: imported documents with a content hash and a positional index.
- **KWIC**: keyword in context; each hit with its left and right context, sortable by neighbouring words.
- **Session log**: `.glossa/session.jsonl`, one line per command, for reproducibility.

## Docs

[Product docs](docs/) · [Build spec](docs/07-build-spec.md) · [Wiki source](docs/wiki/Home.md)

Sister project: [langSuite](https://github.com/ao3575911/langSuite) is the model-driven measurement lab (surprisal, embeddings, semantic change). Glossa is the corpus workbench.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) and [AGENTS.md](AGENTS.md). Before a PR: `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`. Security: [SECURITY.md](SECURITY.md).

## Licence

[MIT](LICENSE)
