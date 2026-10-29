use criterion::{criterion_group, criterion_main, Criterion};
use glossa_index::{build_corpus_index, search_kwic, CorpusIndex, KwicOptions};
use glossa_io::{Document, DocumentKind, ImportBatch};
use indexmap::IndexMap;
use std::hint::black_box;
use std::time::{Duration, Instant};
use tempfile::tempdir;

/// Build synthetic text that tokenises to roughly `target_tokens` tokens.
fn synthetic_text(target_tokens: usize) -> String {
    // Fixed vocabulary sentence (~12 word tokens + punctuation).
    let sentence = "The quick brown fox jumps over the lazy dog near the river. ";
    let toks_per = 12usize;
    let repeats = target_tokens.div_ceil(toks_per);
    sentence.repeat(repeats)
}

fn bench_index_and_kwic(c: &mut Criterion) {
    let mut group = c.benchmark_group("glossa_stage1");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(5));

    // Smaller warm path for criterion; full 10M reported once below.
    group.bench_function("index_1m_tokens", |b| {
        b.iter_custom(|iters| {
            let mut total = Duration::ZERO;
            for _ in 0..iters {
                let dir = tempdir().expect("temp");
                let text = synthetic_text(1_000_000);
                let mut batch = ImportBatch::default();
                batch.documents.push(Document {
                    filename: "synth.txt".into(),
                    source_path: "synth.txt".into(),
                    kind: DocumentKind::Txt,
                    text,
                    metadata: IndexMap::new(),
                    annotations: None,
                });
                let start = Instant::now();
                let stats = build_corpus_index(black_box(&batch), dir.path()).expect("index");
                total += start.elapsed();
                assert!(stats.token_count > 900_000);
            }
            total
        });
    });

    group.finish();

    // One-shot gate numbers printed for the Stage 1 "done when" report.
    {
        let dir = tempdir().expect("temp");
        let text = synthetic_text(10_000_000);
        let mut batch = ImportBatch::default();
        batch.documents.push(Document {
            filename: "synth10m.txt".into(),
            source_path: "synth10m.txt".into(),
            kind: DocumentKind::Txt,
            text,
            metadata: IndexMap::new(),
            annotations: None,
        });
        let t0 = Instant::now();
        let stats = build_corpus_index(&batch, dir.path()).expect("index 10m");
        let index_secs = t0.elapsed().as_secs_f64();
        eprintln!(
            "STAGE1_BENCH index_10m_tokens={:.3}s tokens={} hash={}",
            index_secs, stats.token_count, stats.corpus_hash
        );

        let index = CorpusIndex::open(dir.path()).expect("open");
        // High-frequency node ("fox") plus a sorted query — both must stay under 1s.
        let t1 = Instant::now();
        let hits = search_kwic(
            &index,
            "fox",
            &KwicOptions {
                limit: 100,
                context: 5,
                sort: glossa_index::KwicSort::R1,
                ..Default::default()
            },
        )
        .expect("kwic");
        let kwic_secs = t1.elapsed().as_secs_f64();
        eprintln!(
            "STAGE1_BENCH kwic_fox_sort_r1={:.4}s hits={}",
            kwic_secs,
            hits.len()
        );
        assert!(
            index_secs < 30.0,
            "indexing 10M tokens took {index_secs:.2}s (budget 30s)"
        );
        assert!(kwic_secs < 1.0, "kwic took {kwic_secs:.4}s (budget 1s)");
    }
}

criterion_group!(benches, bench_index_and_kwic);
criterion_main!(benches);
