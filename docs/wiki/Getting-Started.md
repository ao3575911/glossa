# Getting started

## Install (dev)

```bash
cargo install --path crates/glossa-cli
# or
cargo run -p glossa-cli -- --help
```

## Quick start

```bash
glossa init my-project
cd my-project

glossa corpus add path/to/texts --name demo
glossa corpus ls
glossa kwic Alice --corpus demo --sort R1 --context 5 --limit 20

glossa --version --json
GLOSSA_BANNER=1 glossa about
```

## Workspace layout

```
glossa.toml
.glossa/
  workspace.db
  session.jsonl
  corpora/<name>/
```

Every command appends one line to `.glossa/session.jsonl`. JSON output uses a provenance envelope (`glossa_version`, `command`, `params`, hashes, timings, `data`).

## Next

- [Architecture](Architecture)
- [Contributing handbook](Contributing-Handbook)
- Product PRD: `docs/06-PRD.md`
