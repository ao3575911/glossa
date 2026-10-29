use crate::app::{App, RunConfig, Tab};
use glossa_commands::{corpus, init, kwic};
use ratatui::backend::TestBackend;
use ratatui::Terminal;
use std::path::PathBuf;
use tempfile::tempdir;

fn fixture_corpus() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/corpus/alice_excerpt.txt")
}

fn fixture_meta() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/corpus/meta.csv")
}

fn snapshot_config() -> (RunConfig, tempfile::TempDir) {
    let dir = tempdir().expect("tempdir");
    let config = RunConfig {
        // Non-workspace path so snapshots do not pick up the developer's CWD.
        workspace_hint: Some(dir.path().to_path_buf()),
        no_color: true,
    };
    (config, dir)
}

#[test]
fn integration_init_add_kwic_alice_r1() {
    let dir = tempdir().expect("tempdir");
    let (ws, _) =
        init::execute(Some(dir.path().to_path_buf()), Some("fixture".into())).expect("init");
    corpus::add(
        &ws,
        fixture_corpus(),
        Some("alice".into()),
        Some(fixture_meta()),
    )
    .expect("add");
    let res = kwic::execute(
        &ws,
        kwic::KwicRequest {
            query: "Alice".into(),
            corpus: Some("alice".into()),
            regex: false,
            sort: "R1".into(),
            context: 3,
            limit: 5,
        },
    )
    .expect("kwic");
    assert!(!res.hits.is_empty());
    assert_eq!(res.corpus, "alice");
}

#[test]
fn snapshot_workspace_tab() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).expect("terminal");
    let (config, _guard) = snapshot_config();
    let mut app = App::new(config);
    app.set_tab(Tab::Workspace);
    terminal.draw(|f| app.draw(f)).expect("draw");
    insta::assert_snapshot!("tui_workspace_80x24", buffer_string(&terminal));
}

#[test]
fn snapshot_kwic_tab() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).expect("terminal");
    let (config, _guard) = snapshot_config();
    let mut app = App::new(config);
    app.set_tab(Tab::Concordance);
    app.set_kwic_query("Alice");
    terminal.draw(|f| app.draw(f)).expect("draw");
    insta::assert_snapshot!("tui_kwic_80x24", buffer_string(&terminal));
}

#[test]
fn snapshot_stats_tab() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).expect("terminal");
    let (config, _guard) = snapshot_config();
    let mut app = App::new(config);
    app.set_tab(Tab::Stats);
    terminal.draw(|f| app.draw(f)).expect("draw");
    insta::assert_snapshot!("tui_stats_80x24", buffer_string(&terminal));
}

#[test]
fn snapshot_session_tab() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).expect("terminal");
    let (config, _guard) = snapshot_config();
    let mut app = App::new(config);
    app.set_tab(Tab::Session);
    terminal.draw(|f| app.draw(f)).expect("draw");
    insta::assert_snapshot!("tui_session_80x24", buffer_string(&terminal));
}

fn buffer_string(terminal: &Terminal<TestBackend>) -> String {
    let buf = terminal.backend().buffer();
    let area = buf.area;
    let mut out = String::new();
    for y in 0..area.height {
        for x in 0..area.width {
            out.push_str(buf[(x, y)].symbol());
        }
        out.push('\n');
    }
    out
}
