# Product requirements document (PRD)

## 1. Problem
Researchers move data between 4–6 tools with incompatible formats; glue scripts are ad hoc and undocumented; GUI tools can't be automated; Python NLP libraries can't do corpus-linguistic statistics without custom code; commercial web tools cost money and need the internet. Result: lost time, unreproducible findings, and exclusion of field and low-bandwidth researchers.

## 2. Personas
| Persona | Context | Primary need |
|---|---|---|
| **Maya, PhD corpus linguist** | Builds 20M-token corpora, uses AntConc + R | Fast KWIC/collocations/keyness, exact stats, reproducible export to R |
| **Tomás, field linguist** | Documents an under-described language, patchy internet | Offline IGT, lexicon, ELAN interop on a laptop |
| **Priya, NLP engineer** | Builds pipelines, uses spaCy | Pipeable CLI with JSON, UD parsing without Python env pain |
| **Dr. Lee, lecturer** | Teaches intro corpus/syntax | Something students install in one command and explore without coding |
| **Sam, translator/advanced learner** | Checks usage and collocations | Quick concordance and collocations on own texts |

## 3. User stories (MVP)
- As Maya, I import a folder of DOCX/PDF/TXT with a metadata CSV and get an indexed corpus in under a minute per million tokens.
- As Maya, I run `glossa kwic 'lemma="make" [pos="NOUN"]'` and sort by right context.
- As Maya, I compute collocates with logDice and keyness vs a reference corpus, and export a tidy CSV for R.
- As Priya, I `cat text.txt | glossa parse --lang en --json | jq` and get UD tokens.
- As Priya, I write a CoNLL-U file and query it with a tree pattern.
- As Lee, I open `glossa`, pick a corpus and browse word → concordance → parse tree with arrows only.
- As anyone, I run `glossa log methods` and get a draft methods paragraph plus BibTeX for every tool, model and corpus used.
- As anyone, I re-run `glossa replay session.jsonl` and get byte-identical outputs.

## 4. Functional requirements (MVP)
| ID | Requirement | Module |
|---|---|---|
| F1 | Import TXT, HTML, DOCX, PDF (text layer), CoNLL-U, TEI, CHAT, ELAN EAF; auto-detect encoding; per-document metadata | 1 |
| F2 | Positional index (word, lemma, POS, features) with sub-second KWIC over 100M tokens | 1 |
| F3 | Query: plain, regex, CQL subset (token attributes, sequences, repetition, `within <s>`) | 1 |
| F4 | Frequency lists, n-grams (1–5), collocations (MI, MI3, t-score, log-likelihood, logDice) with window and min-freq | 1 |
| F5 | Keyness (log-likelihood, %DIFF, log ratio) vs reference corpus/subcorpus | 1 |
| F6 | Dispersion (range, Juilland's D, DP) and dispersion plot | 1 |
| F7 | Subcorpora defined by metadata filters; side-by-side comparison | 1 |
| F8 | Tokenise, sentence-split, lemmatise, UPOS/XPOS/feats, UD dependency parse, offline models per language | 2 |
| F9 | Tree viewer (ASCII/Unicode dependency trees) | 2 |
| F10 | Treebank query over CoNLL-U (Grew-like/Semgrex-like pattern subset; assumption: own simplified syntax first) | 2 |
| F11 | Morphological segmentation via pluggable analysers (e.g. HFST/FST transducers as plugins) | 2 |
| F12 | Workspaces (`glossa.toml` + `.glossa/` dir), saved queries, session log JSONL | 11 |
| F13 | BibTeX and methods-section generator from log | 11 |
| F14 | Exports: CSV, TSV, JSON/NDJSON, CoNLL-U, TEI, EAF, Parquet (pandas/R-ready) | 11 |
| F15 | Plugin API v0 (WASM components + external-process protocol) | 11 |
| F16 | Every command: `--json`, stdin/stdout piping, stable exit codes | all |

## 5. Non-functional requirements
- **Performance:** cold start < 150 ms; KWIC over 100M tokens < 1 s; index ≥ 1M tokens/s (tokenise-only) on a 2022 laptop.
- **Portability:** Linux x86_64/aarch64, macOS arm64/x86_64, Windows x86_64; single static binary < 30 MB without models.
- **Offline:** zero network calls unless the user runs `glossa models pull` / `glossa data pull`.
- **Reproducibility:** deterministic outputs (fixed seeds, stable sort); each result header records glossa version, model SHA-256, corpus hash, params.
- **Accessibility:** 100% keyboard; high-contrast + colour-blind-safe themes; `--plain` mode with linear output for screen readers; no information conveyed by colour alone.
- **Unicode:** NFC normalisation option, grapheme-aware widths, RTL display tested (Arabic, Hebrew).
- **Privacy:** no telemetry by default.
- **AI:** optional, labelled `[AI]`, local or user-supplied endpoint, never in default pipelines.

## 6. Scope
| Stage | Modules | Scope |
|---|---|---|
| MVP (weeks 1–6) | 1, 2, 11 | Above F1–F16 |
| v0.2 | 5 | Stats + charts (needed by everything after) |
| v0.3 | 3 | Phonetics/phonology + Praat bridge |
| v0.4 | 4 | Annotation/fieldwork, IGT, lexicon |
| v0.5 | 6 | Lexicography/semantics |
| v0.6 | 8 | Typology + language ID |
| v0.7 | 7 | Historical/comparative |
| v0.8 | 9 | Socioling/discourse |
| v0.9 | 10 | Experimental/psycholing |
| v1.0 | all | Stable plugin API, docs, packaging |

## 7. Success metrics
- MVP: new user installs and runs first KWIC in < 5 minutes without docs (5 of 5 testers).
- Collocation/keyness numbers match reference implementation outputs (hand-computed and AntConc cross-check) to 4 decimal places.
- UD parsing within 2 LAS points of the source model's published score on the UD test set.
- 6 months post-launch (assumption-level targets): 1,000 GitHub stars, 10 third-party plugins, cited in ≥ 3 papers.

## 8. Risks
| Risk | Mitigation |
|---|---|
| Neural parsing in Rust without Python | ONNX Runtime via `ort`; fallback external-process plugin to Stanza/spaCy |
| Model licences (e.g. many UDPipe models are CC BY-NC-SA) | Never bundle; download on demand with licence shown and recorded |
| PDF/DOCX extraction quality | Text-layer only in MVP; warn on scanned PDFs |
| Scope sprawl across 11 modules | Hard stage gates with "done when" tests |
| CQL dialect expectations | Document supported subset; follow CWB/Sketch Engine syntax where implemented |
| TUI accessibility limits with screen readers | `--plain` linear mode is a first-class output, tested with NVDA/Orca |

## 9. Open questions
1. Which treebank query syntax to align with long-term: Grew, Semgrex or PML-TQ?
2. Ship a small default English model in the binary, or always download?
3. R bridge via Rscript subprocess or embedded? (assumption: subprocess)
4. Governance: solo project vs foundation/academic partner?
5. Dual licence for plugins, or MIT everywhere?
