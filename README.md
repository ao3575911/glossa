# Glossa

**Glossa: every linguist's research tool, in one fast terminal workspace.**

Glossa is an offline-first, single-binary command-line tool with a full-screen terminal UI (TUI) that bundles corpus, morphosyntax, phonetics, fieldwork, statistics, historical, typological and psycholinguistic tools in one keyboard-driven workspace. It is built for linguists, field workers, researchers, translators, advanced learners and NLP developers who today stitch together AntConc, Praat, ELAN, Python scripts and websites. Every tool runs both in the TUI and as a pipeable `glossa` subcommand with JSON output, and every result records the version and parameters that produced it.

> Status: design stage. Planned stack: Rust + ratatui, single static binary.

## Modules
1. **Corpus linguistics** (MVP): import, KWIC with regex/CQL, frequency, n-grams, collocations, keyness, dispersion, subcorpora
2. **Morphosyntax** (MVP): tokenise, lemmatise, POS, UD parsing, morphology, tree viewer, treebank query
3. Phonetics and phonology: IPA, G2P, inventories, features, syllables, Praat bridge
4. Annotation and fieldwork: span tagging, kappa, IGT editor, lexicon builder, time-aligned transcription
5. Statistics and visualisation: chi-square, Fisher, LL, mixed effects via R/Python, terminal charts
6. Lexicography and semantics: word sketches, sense clustering, embeddings, WordNet
7. Historical and comparative: cognates, sound correspondences, sound-change simulator, etymology
8. Typology: language ID, transliteration, WALS/Glottolog/PHOIBLE, parallel alignment
9. Sociolinguistics and discourse: variation, discourse markers, CA transcripts, stylometry
10. Experimental and psycholinguistics: stimulus builder, frequency/concreteness/AoA norms
11. **Research workflow** (MVP): workspaces, saved queries, reproducible log, methods + BibTeX, plugins, exports

## MVP scope
Modules 1, 2 and 11, in six stages over about 6 weeks for one developer: foundations; import + index + KWIC; stats core; annotation + CQL; TUI; workflow + plugins. Each later module ships as its own stage with a "done when" test. See [build spec](docs/07-build-spec.md#11-staged-milestones).

## Docs
1. [One-liner](docs/01-one-liner.md)
2. [Definition](docs/02-definition.md)
3. [Executive statement](docs/03-exec-statement.md)
4. [Overview and comparison](docs/04-overview.md)
5. [Vision and mission](docs/05-vision-mission.md)
6. [PRD](docs/06-PRD.md)
7. [Build plan / technical spec](docs/07-build-spec.md)
8. [Recommended next prompt](docs/08-next-prompt.md)

## License
MIT
