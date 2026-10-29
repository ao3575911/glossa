# Glossa

**Glossa: every linguist's research tool, in one fast terminal workspace.**

Offline-first linguistics CLI/TUI (`glossa`). Stage 0–1: workspaces, import, positional index, and KWIC.

| | |
| --- | --- |
| License | [MIT](./LICENSE) |
| Contribute | [CONTRIBUTING.md](./CONTRIBUTING.md) |
| Security | [SECURITY.md](./SECURITY.md) |
| Support | [SUPPORT.md](./SUPPORT.md) |
| Agents | [AGENTS.md](./AGENTS.md) |
| Brand | [`brand/`](./brand/) |

## Install (dev)

```bash
cargo install --path crates/glossa-cli
# or
cargo run -p glossa-cli -- --help
```

## Quick start

```bash
# Create a workspace
glossa init my-project
cd my-project

# Import a folder or file (TXT, HTML, DOCX, PDF text-layer, CoNLL-U)
glossa corpus add ../tests/fixtures/corpus/alice_excerpt.txt --name alice \
  --meta ../tests/fixtures/corpus/meta.csv

glossa corpus ls
glossa corpus info alice

# Concordance (plain word, case-insensitive on lowercased forms)
glossa kwic Alice --corpus alice --sort R1 --context 5 --limit 20

# Regex over surface forms
glossa kwic 'Rab+it' --regex --corpus alice

# Machine-readable envelope (every command)
glossa --version --json
glossa kwic Alice --corpus alice --json

# Branding (quiet by default; compact banner on request)
glossa about
GLOSSA_BANNER=1 glossa
```

JSON results use a provenance envelope:

```json
{
  "glossa_version": "0.1.0",
  "command": "kwic",
  "params": { "...": "..." },
  "corpus_hash": "...",
  "model_hashes": {},
  "started_at": "...",
  "duration_ms": 12,
  "data": { "...": "..." }
}
```

Every command appends one line to `.glossa/session.jsonl`.

## Workspace layout

```
glossa.toml
.glossa/
  workspace.db      # SQLite metadata
  session.jsonl     # append-only command log
  corpora/<name>/   # manifest + columnar index
```

## Crates

| Crate | Role |
|---|---|
| `glossa-cli` | Binary (`glossa`) |
| `glossa-core` | Result envelope / output mode |
| `glossa-workspace` | `init`, SQLite, session log |
| `glossa-io` | Importers |
| `glossa-nlp` | UAX #29 tokeniser + sentence split |
| `glossa-index` | Positional index + KWIC |
| `glossa-tui` / `glossa-stats` / `glossa-plugin` | Stubs (later stages) |

## Development

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo bench -p glossa-index --bench index_kwic
```

CI runs on Ubuntu, macOS, and Windows (`fmt`, `clippy -D warnings`, `test`). Security and quality jobs run `cargo-deny`, `cargo-audit`, and CodeQL. Releases use cargo-dist (`dist-workspace.toml`).

## Docs

Product docs:

1. [One-liner](docs/01-one-liner.md)
2. [Definition](docs/02-definition.md)
3. [Executive statement](docs/03-exec-statement.md)
4. [Overview and comparison](docs/04-overview.md)
5. [Vision and mission](docs/05-vision-mission.md)
6. [PRD](docs/06-PRD.md)
7. [Build plan / technical spec](docs/07-build-spec.md)
8. [Recommended next prompt](docs/08-next-prompt.md)

Operations:

- [Wiki source](docs/wiki/Home.md) · [Projects board guide](docs/projects.md) · [GitHub Pages site](docs/pages/index.html)
- Labels: [`.github/labels.yml`](.github/labels.yml) · Issue / Discussion / PR templates under [`.github/`](.github/)

## Community

- [Code of conduct](CODE_OF_CONDUCT.md)
- [Contributing](CONTRIBUTING.md)
- [Discussions](https://github.com/ao3575911/glossa/discussions) (enable in settings if the link 404s)
- Issues use labelled templates (`type:*`, `area:*`, `status:triage`)

## License

MIT
