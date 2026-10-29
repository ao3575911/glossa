# Architecture

High-level design (see `docs/07-build-spec.md` for the full specification).

## Crates

| Crate | Role |
| --- | --- |
| `glossa-cli` | Binary (`glossa`) |
| `glossa-core` | Result envelope / output mode |
| `glossa-workspace` | `init`, SQLite, session log |
| `glossa-io` | Importers (TXT, HTML, DOCX, PDF text, CoNLL-U) |
| `glossa-nlp` | UAX #29 tokeniser + sentence split |
| `glossa-index` | Positional index + KWIC |
| `glossa-tui` / `glossa-stats` / `glossa-plugin` | Stubs (later stages) |

## Data flow

```
User → CLI/TUI → core → io / nlp / index / workspace
                         ↓
                   provenance envelope + session.jsonl
```

## Index sketch

Per attribute: FST vocabulary, `u32` token-id columns (memmap2), delta+varint postings, sentence/document boundaries, SHA-256 corpus hash.

## Decisions

Accepted deviations live in `docs/decisions/`. Add a new ADR when changing intentional behaviour.
