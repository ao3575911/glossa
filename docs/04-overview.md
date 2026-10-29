# Overview

## Capabilities (11 modules)

| # | Module | Highlights | Stage |
|---|---|---|---|
| 1 | Corpus linguistics | Import TXT/PDF/DOCX/HTML/CoNLL-U/TEI/CHAT/ELAN; KWIC with regex + CQL; frequency, n-grams, collocations (MI, t-score, log-likelihood, logDice), keyness; dispersion; subcorpus comparison | **MVP** |
| 2 | Morphosyntax | Tokenise, lemmatise, POS, UD dependency parsing; morphological segmentation (incl. agglutinative/polysynthetic via plugins); tree viewer; treebank query | **MVP** |
| 11 | Research workflow | Workspaces, saved queries, reproducible session log, BibTeX + methods-section generator, plugin API, exports (CSV, JSON, CoNLL-U, TEI, ELAN, R/pandas) | **MVP** |
| 3 | Phonetics/phonology | IPA keyboard, G2P, inventories, distinctive features, natural classes, syllabification, stress, rhyme, Praat bridge (TextGrid, formants, pitch, scripts) | Later |
| 4 | Annotation/fieldwork | Span tagging, Cohen's/Fleiss' kappa, Leipzig-style IGT editor, lexicon builder, time-aligned transcription | Later |
| 5 | Stats/visualisation | Chi-square, Fisher, LL; mixed-effects via R/Python bridge; terminal charts; SVG/PNG export | Later |
| 6 | Lexicography/semantics | Word sketches, sense clustering, embedding neighbours, WordNet | Later |
| 7 | Historical/comparative | Cognate detection, sound correspondences, sound-change simulator, cached Wiktionary etymology | Later |
| 8 | Typology | Language ID, transliteration, WALS/Glottolog/PHOIBLE, parallel alignment | Later |
| 9 | Socioling/discourse | Variation by speaker metadata, discourse markers, CA transcripts, stylometry | Later |
| 10 | Experimental/psycholing | Stimulus builder matched on frequency/length/neighbourhood; SUBTLEX, concreteness, AoA norms | Later |

## Moving through the TUI

- **Layout:** left sidebar (project → corpora → saved queries), main pane (active tool), bottom pane (session log / status), command palette on `:`.
- **Keys:** `Ctrl-P` fuzzy palette of every tool; `Tab` cycles panes; `/` search inside a result; `Enter` drills down (word → KWIC → sentence → tree); `y` copies the equivalent CLI command; `e` exports; `?` contextual help; `F2` toggles high-contrast theme.
- **Round-trip:** every TUI view shows the exact `glossa …` command that reproduces it, so exploration becomes a script.

## Comparison with existing tools

Facts as of late 2026; licences/platforms checked against public project pages, details may change.

| Tool | Type | Strength | Gap Glossa fills |
|---|---|---|---|
| AntConc (Laurence Anthony) | Free desktop GUI, Win/macOS/Linux | Easy concordance, collocates, keywords | Not scriptable, no parsing, no JSON pipeline |
| Sketch Engine (Lexical Computing) | Commercial web service | Huge pre-loaded corpora, word sketches, CQL | Subscription, online, your data goes to a server |
| spaCy (Explosion) | Python library, MIT | Fast production NLP pipelines | Code only, no corpus-linguistics stats or UI |
| Stanza (Stanford NLP) | Python library, Apache-2.0 | UD-trained neural pipelines for 60+ languages | Python + PyTorch runtime, code only |
| NLTK | Python library, Apache-2.0 | Teaching, classic algorithms, corpora readers | Slow, dated models, code only |
| Praat (Boersma & Weenink) | Desktop GUI + scripting, GPL | Gold standard acoustic analysis | Phonetics only; Glossa bridges to it, not replaces it |
| ELAN (MPI Nijmegen / The Language Archive) | Desktop GUI, GPL | Time-aligned multi-tier annotation | No corpus stats; Glossa reads/writes EAF |
| FLEx (SIL FieldWorks) | Desktop GUI, primarily Windows (Linux builds exist), open source | Lexicon + interlinear text for fieldwork | Heavy, GUI-only; Glossa imports/exports its data via LIFT (assumption: LIFT is the stable interchange path) |

**Position:** Glossa does not try to beat Praat at acoustics or Sketch Engine at corpus size. It is the scriptable, offline hub that unifies them.
