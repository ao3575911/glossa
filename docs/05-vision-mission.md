# Vision and mission

**Vision:** any linguist, anywhere, can go from raw language data to a reproducible, citable analysis in one tool, with no internet connection and no programming required.

**Mission:** build a fast, open, offline terminal workspace where every linguistic analysis is interactive, scriptable and exactly re-runnable.

## Guiding principles
1. **One action, two faces.** Every feature is a TUI screen *and* a CLI subcommand with JSON output. No TUI-only features.
2. **Reproducible by default.** Every result carries tool version, model hash, corpus hash and parameters; the session log can regenerate it.
3. **Offline first.** Nothing requires the network after a model/data download; downloads are explicit and checksummed.
4. **Interoperate, don't trap.** Read and write the community's formats (CoNLL-U, TEI, EAF, TextGrid, LIFT, CSV). No proprietary project format.
5. **Respect the language, not just English.** Unicode-correct, script-aware, and extensible to under-resourced and morphologically rich languages.

## What Glossa is NOT
- Not a GUI or web app, and not a cloud service.
- Not a replacement for Praat's signal processing or R's full statistics: it bridges to them.
- Not an AI chatbot. Optional AI helpers are clearly labelled, off by default, and never needed for any result.
- Not a model-training framework. It runs models; training stays in spaCy/Stanza/UDPipe.
- Not a language-learning app.
