use crate::theme::Theme;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use glossa_commands::corpus as corpus_cmd;
use glossa_commands::init as init_cmd;
use glossa_commands::kwic::{self, KwicRequest};
use glossa_core::GLOSSA_VERSION;
use glossa_index::KwicHit;
use glossa_workspace::{CorpusRecord, SessionEvent, Workspace};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState, Tabs, Wrap};
use ratatui::{Frame, Terminal};
use std::io;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

const TABS: [&str; 4] = ["Workspace", "Concordance", "Stats", "Session"];

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Workspace = 0,
    Concordance = 1,
    Stats = 2,
    Session = 3,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum InputMode {
    Normal,
    Command,
    Query,
}

enum BackgroundMsg {
    CorpusAdd(Result<corpus_cmd::AddResult, String>),
}

pub struct RunConfig {
    pub workspace_hint: Option<PathBuf>,
    pub no_color: bool,
}

pub struct App {
    theme: Theme,
    tab: Tab,
    workspace: Option<Workspace>,
    workspace_root: String,
    corpora: Vec<CorpusRecord>,
    corpus_table: TableState,
    selected_corpus: Option<usize>,
    corpus_detail: String,
    kwic_query: String,
    kwic_regex: bool,
    kwic_sort: String,
    kwic_context: usize,
    kwic_limit: usize,
    kwic_hits: Vec<KwicHit>,
    kwic_table: TableState,
    kwic_detail: String,
    session_lines: Vec<String>,
    status: String,
    input_mode: InputMode,
    input_buffer: String,
    busy: bool,
    bg_rx: Option<mpsc::Receiver<BackgroundMsg>>,
    should_quit: bool,
    show_help: bool,
}

#[cfg(test)]
impl App {
    pub fn set_tab(&mut self, tab: Tab) {
        self.tab = tab;
    }

    pub fn set_kwic_query(&mut self, q: impl Into<String>) {
        self.kwic_query = q.into();
    }
}

impl App {
    pub fn new(config: RunConfig) -> Self {
        let theme = Theme::dark(config.no_color);
        let mut app = Self {
            theme,
            tab: Tab::Workspace,
            workspace: None,
            workspace_root: String::new(),
            corpora: Vec::new(),
            corpus_table: TableState::default(),
            selected_corpus: None,
            corpus_detail: String::new(),
            kwic_query: String::new(),
            kwic_regex: false,
            kwic_sort: String::new(),
            kwic_context: 5,
            kwic_limit: 100,
            kwic_hits: Vec::new(),
            kwic_table: TableState::default(),
            kwic_detail: String::new(),
            session_lines: Vec::new(),
            status: format!("Glossa {GLOSSA_VERSION} — Tab switch · q quit · ? help"),
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
            busy: false,
            bg_rx: None,
            should_quit: false,
            show_help: false,
        };
        app.try_open_workspace(config.workspace_hint);
        app
    }

    fn try_open_workspace(&mut self, hint: Option<PathBuf>) {
        let start = hint
            .or_else(|| std::env::current_dir().ok())
            .unwrap_or_else(|| PathBuf::from("."));
        match Workspace::open(&start) {
            Ok(ws) => {
                self.workspace_root = ws.root.display().to_string();
                self.workspace = Some(ws);
                self.refresh_corpora();
                self.refresh_session();
                self.status = format!("Workspace: {}", self.workspace_root);
            }
            Err(_) => {
                self.status = "No workspace — press i or :init <dir> to create one".to_string();
            }
        }
    }

    fn refresh_corpora(&mut self) {
        if let Some(ws) = &self.workspace {
            self.corpora = corpus_cmd::list(ws).unwrap_or_default();
            if self.corpora.is_empty() {
                self.selected_corpus = None;
                self.corpus_detail.clear();
            } else {
                let idx = self
                    .selected_corpus
                    .unwrap_or(0)
                    .min(self.corpora.len() - 1);
                self.selected_corpus = Some(idx);
                self.corpus_table.select(Some(idx));
                self.update_corpus_detail(idx);
            }
        }
    }

    fn update_corpus_detail(&mut self, idx: usize) {
        if let Some(c) = self.corpora.get(idx) {
            self.corpus_detail = format!(
                "name: {}\ntokens: {}\ndocs: {}\nsentences: {}\nhash: {}\ncreated: {}",
                c.name,
                c.token_count,
                c.doc_count,
                c.sentence_count,
                c.corpus_hash,
                c.created_at.to_rfc3339()
            );
        }
    }

    fn refresh_session(&mut self) {
        self.session_lines.clear();
        if let Some(ws) = &self.workspace {
            let path = ws.session_log().path();
            if let Ok(text) = std::fs::read_to_string(path) {
                self.session_lines = text.lines().map(str::to_string).collect();
            }
        }
    }

    fn run_kwic(&mut self) {
        if self.kwic_query.trim().is_empty() {
            self.status = "Enter a query (/ or Enter on Concordance tab)".into();
            return;
        }
        let Some(ws) = self.workspace.as_ref() else {
            self.status = "Open a workspace first".into();
            return;
        };
        let corpus = self
            .selected_corpus
            .and_then(|i| self.corpora.get(i).map(|c| c.name.clone()));
        let req = KwicRequest {
            query: self.kwic_query.clone(),
            corpus,
            regex: self.kwic_regex,
            sort: self.kwic_sort.clone(),
            context: self.kwic_context,
            limit: self.kwic_limit,
        };
        let kwic_result = kwic::execute(ws, req);
        match kwic_result {
            Ok(res) => {
                let n = res.hit_count;
                let corpus = res.corpus.clone();
                self.kwic_hits = res.hits;
                self.kwic_table.select(if n > 0 { Some(0) } else { None });
                self.update_kwic_detail(0);
                self.status = format!("KWIC: {n} hits (corpus {corpus})");
                if let Some(ws) = self.workspace.as_ref() {
                    let _ = ws.session_log().append(&SessionEvent {
                        ts: chrono::Utc::now(),
                        command: "kwic".into(),
                        args: serde_json::json!({ "query": self.kwic_query, "via": "tui" }),
                        ok: true,
                        error: None,
                        duration_ms: None,
                    });
                }
                self.refresh_session();
            }
            Err(e) => self.status = format!("KWIC error: {e:#}"),
        }
    }

    fn update_kwic_detail(&mut self, idx: usize) {
        if let Some(h) = self.kwic_hits.get(idx) {
            self.kwic_detail = format!(
                "doc: {}\nposition: {}\nleft: {}\n>> {} <<\nright: {}",
                h.doc,
                h.position,
                h.left.join(" "),
                h.match_text,
                h.right.join(" ")
            );
        } else {
            self.kwic_detail.clear();
        }
    }

    fn do_init(&mut self, dir: &str) {
        let path = if dir.is_empty() {
            PathBuf::from(".")
        } else {
            PathBuf::from(dir)
        };
        match init_cmd::execute(Some(path.clone()), None) {
            Ok((ws, data)) => {
                self.workspace = Some(ws);
                self.workspace_root = data.root;
                self.refresh_corpora();
                self.refresh_session();
                self.status = format!("Initialized workspace '{}'", data.name);
            }
            Err(e) => self.status = format!("init failed: {e:#}"),
        }
    }

    fn start_corpus_add(&mut self, path: &str) {
        if self.busy {
            self.status = "Busy — wait for current import".into();
            return;
        }
        let Some(ws) = self.workspace.as_ref() else {
            self.status = "Open a workspace first".into();
            return;
        };
        let root = ws.root.clone();
        let path = PathBuf::from(path);
        let (tx, rx) = mpsc::channel();
        self.bg_rx = Some(rx);
        self.busy = true;
        self.status = format!("Importing {}…", path.display());
        thread::spawn(move || {
            let result = Workspace::open(&root)
                .map_err(|e| format!("{e}"))
                .and_then(|ws| {
                    corpus_cmd::add(&ws, path, None, None).map_err(|e| format!("{e:#}"))
                });
            let _ = tx.send(BackgroundMsg::CorpusAdd(result));
        });
    }

    fn poll_background(&mut self) {
        let msg = self.bg_rx.as_ref().and_then(|rx| rx.try_recv().ok());
        if let Some(BackgroundMsg::CorpusAdd(result)) = msg {
            self.busy = false;
            self.bg_rx = None;
            match result {
                Ok(add) => {
                    self.status =
                        format!("Added corpus '{}' ({} tokens)", add.name, add.token_count);
                    if let Some(ws) = &self.workspace {
                        let _ = ws.session_log().append(&SessionEvent {
                            ts: chrono::Utc::now(),
                            command: "corpus add".into(),
                            args: serde_json::json!({ "name": add.name, "via": "tui" }),
                            ok: true,
                            error: None,
                            duration_ms: None,
                        });
                    }
                    self.refresh_corpora();
                    self.refresh_session();
                }
                Err(e) => self.status = format!("Import failed: {e}"),
            }
        }
    }

    fn export_kwic_csv(&mut self) {
        if self.kwic_hits.is_empty() {
            self.status = "No KWIC rows to export".into();
            return;
        }
        let path = PathBuf::from("glossa_kwic_export.csv");
        let mut out = String::from("doc,position,left,match,right\n");
        for h in &self.kwic_hits {
            out.push_str(&format!(
                "{},{},\"{}\",\"{}\",\"{}\"\n",
                escape_csv(&h.doc),
                h.position,
                escape_csv(&h.left.join(" ")),
                escape_csv(&h.match_text),
                escape_csv(&h.right.join(" ")),
            ));
        }
        if std::fs::write(&path, out).is_ok() {
            self.status = format!("Exported CSV to {}", path.display());
        } else {
            self.status = "Could not write export file".into();
        }
    }

    fn handle_command(&mut self, line: &str) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            return;
        }
        match parts[0] {
            "init" => {
                let dir = parts.get(1).copied().unwrap_or(".");
                self.do_init(dir);
            }
            "add" => {
                if let Some(p) = parts.get(1) {
                    self.start_corpus_add(p);
                } else {
                    self.status = "Usage: :add <path>".into();
                }
            }
            "kwic" => {
                if let Some(q) = parts.get(1) {
                    self.kwic_query = (*q).to_string();
                    self.run_kwic();
                }
            }
            _ => self.status = format!("Unknown command: {}", parts[0]),
        }
    }

    pub fn on_key(&mut self, key: KeyEvent) {
        if self.show_help && key.code != KeyCode::Char('?') {
            self.show_help = false;
        }

        if self.input_mode != InputMode::Normal {
            match key.code {
                KeyCode::Enter => {
                    let buf = std::mem::take(&mut self.input_buffer);
                    let mode = self.input_mode;
                    self.input_mode = InputMode::Normal;
                    match mode {
                        InputMode::Query => {
                            self.kwic_query = buf;
                            self.run_kwic();
                        }
                        InputMode::Command => self.handle_command(&buf),
                        InputMode::Normal => {}
                    }
                    return;
                }
                KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                    self.input_buffer.clear();
                    return;
                }
                KeyCode::Backspace => {
                    self.input_buffer.pop();
                    return;
                }
                KeyCode::Char(c) => {
                    self.input_buffer.push(c);
                    return;
                }
                _ => return,
            }
        }

        match key.code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('?') => self.show_help = !self.show_help,
            KeyCode::Tab => {
                self.tab = match self.tab {
                    Tab::Workspace => Tab::Concordance,
                    Tab::Concordance => Tab::Stats,
                    Tab::Stats => Tab::Session,
                    Tab::Session => Tab::Workspace,
                };
            }
            KeyCode::BackTab => {
                self.tab = match self.tab {
                    Tab::Workspace => Tab::Session,
                    Tab::Concordance => Tab::Workspace,
                    Tab::Stats => Tab::Concordance,
                    Tab::Session => Tab::Stats,
                };
            }
            KeyCode::Char('1') => self.tab = Tab::Workspace,
            KeyCode::Char('2') => self.tab = Tab::Concordance,
            KeyCode::Char('3') => self.tab = Tab::Stats,
            KeyCode::Char('4') => self.tab = Tab::Session,
            KeyCode::Char('r') => {
                self.refresh_corpora();
                self.refresh_session();
                self.status = "Refreshed".into();
            }
            KeyCode::Char(':') | KeyCode::Char('i') => {
                self.input_mode = InputMode::Command;
                self.input_buffer = if key.code == KeyCode::Char('i') {
                    "init .".into()
                } else {
                    String::new()
                };
            }
            KeyCode::Char('a') if self.tab == Tab::Workspace => {
                self.input_mode = InputMode::Command;
                self.input_buffer = "add ".into();
            }
            KeyCode::Char('/') if self.tab == Tab::Concordance => {
                self.input_mode = InputMode::Query;
                self.input_buffer = self.kwic_query.clone();
            }
            KeyCode::Enter if self.tab == Tab::Concordance => self.run_kwic(),
            KeyCode::Char('x') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.kwic_regex = !self.kwic_regex;
                self.status = format!("Regex: {}", self.kwic_regex);
            }
            KeyCode::Char('e') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.export_kwic_csv();
            }
            KeyCode::Char('s') if self.tab == Tab::Concordance => {
                self.kwic_sort = match self.kwic_sort.as_str() {
                    "" => "L1".into(),
                    "L1" => "R1".into(),
                    "R1" => "R2".into(),
                    _ => String::new(),
                };
                self.status = format!(
                    "Sort: {}",
                    if self.kwic_sort.is_empty() {
                        "none"
                    } else {
                        &self.kwic_sort
                    }
                );
            }
            KeyCode::Char('j') | KeyCode::Down => self.table_next(1),
            KeyCode::Char('k') | KeyCode::Up => self.table_next(-1),
            KeyCode::Char('-') if self.tab == Tab::Concordance => {
                self.kwic_context = self.kwic_context.saturating_sub(1).max(1);
            }
            KeyCode::Char('=') | KeyCode::Char('+') if self.tab == Tab::Concordance => {
                self.kwic_context = (self.kwic_context + 1).min(20);
            }
            _ => {}
        }
    }

    fn table_next(&mut self, delta: i32) {
        match self.tab {
            Tab::Workspace => {
                if self.corpora.is_empty() {
                    return;
                }
                let i = self.corpus_table.selected().unwrap_or(0) as i32;
                let n = self.corpora.len() as i32;
                let next = ((i + delta).rem_euclid(n)) as usize;
                self.corpus_table.select(Some(next));
                self.selected_corpus = Some(next);
                self.update_corpus_detail(next);
            }
            Tab::Concordance => {
                if self.kwic_hits.is_empty() {
                    return;
                }
                let i = self.kwic_table.selected().unwrap_or(0) as i32;
                let n = self.kwic_hits.len() as i32;
                let next = ((i + delta).rem_euclid(n)) as usize;
                self.kwic_table.select(Some(next));
                self.update_kwic_detail(next);
            }
            _ => {}
        }
    }

    pub fn draw(&mut self, f: &mut Frame<'_>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(8),
                Constraint::Length(3),
                Constraint::Length(1),
            ])
            .split(f.area());

        let tab_idx = self.tab as usize;
        let tabs = Tabs::new(TABS)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(Span::styled(
                        format!(" glossa {GLOSSA_VERSION} "),
                        self.theme.title(),
                    ))
                    .style(Style::default().fg(self.theme.fg).bg(self.theme.bg)),
            )
            .select(tab_idx)
            .style(Style::default().fg(self.theme.muted))
            .highlight_style(self.theme.title());
        f.render_widget(tabs, chunks[0]);

        match self.tab {
            Tab::Workspace => self.draw_workspace(f, chunks[1]),
            Tab::Concordance => self.draw_kwic(f, chunks[1]),
            Tab::Stats => self.draw_stats(f, chunks[1]),
            Tab::Session => self.draw_session(f, chunks[1]),
        }

        let footer = if self.show_help {
            "q quit · Tab tabs · i/:init · a/:add path · / query · Enter search · j/k move · s sort · +/- context · ^x regex · ^e CSV · r refresh"
        } else if self.input_mode != InputMode::Normal {
            "Esc cancel · Enter submit"
        } else {
            "q quit · ? help · 1-4 tabs · Workspace: i init a add · Concordance: / Enter s sort · ^e export"
        };
        let footer_p = Paragraph::new(footer).style(self.theme.footer());
        f.render_widget(footer_p, chunks[2]);

        let mut status = self.status.clone();
        if self.busy {
            status.push_str(" [busy]");
        }
        if self.input_mode != InputMode::Normal {
            status = format!(":{}{}", self.input_buffer, "█");
        }
        let status_p = Paragraph::new(status)
            .style(Style::default().fg(self.theme.rule))
            .wrap(Wrap { trim: true });
        f.render_widget(status_p, chunks[3]);
    }

    fn draw_workspace(&mut self, f: &mut Frame<'_>, area: Rect) {
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
            .split(area);

        let rows: Vec<Row> = self
            .corpora
            .iter()
            .map(|c| {
                Row::new(vec![
                    Cell::from(c.name.clone()),
                    Cell::from(c.doc_count.to_string()),
                    Cell::from(c.token_count.to_string()),
                ])
            })
            .collect();
        let table = Table::new(
            rows,
            [
                Constraint::Percentage(40),
                Constraint::Percentage(20),
                Constraint::Percentage(40),
            ],
        )
        .header(Row::new(vec!["Corpus", "Docs", "Tokens"]).style(self.theme.title()))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Corpora ")
                .style(Style::default().fg(self.theme.fg).bg(self.theme.bg)),
        )
        .row_highlight_style(self.theme.row_selected())
        .highlight_symbol("▸ ");
        f.render_stateful_widget(table, cols[0], &mut self.corpus_table);

        let meta = if self.workspace_root.is_empty() {
            "No workspace open.\n\nPress i or :init <dir>".to_string()
        } else {
            format!("root:\n{}\n\n{}", self.workspace_root, self.corpus_detail)
        };
        let p = Paragraph::new(meta)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Metadata ")
                    .style(Style::default().fg(self.theme.fg).bg(self.theme.bg)),
            )
            .wrap(Wrap { trim: true });
        f.render_widget(p, cols[1]);
    }

    fn draw_kwic(&mut self, f: &mut Frame<'_>, area: Rect) {
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Min(4),
                Constraint::Length(6),
            ])
            .split(area);

        let header = format!(
            "query: {} | corpus: {} | sort: {} | ctx: {} | limit: {} | regex: {}",
            if self.kwic_query.is_empty() {
                "(none)"
            } else {
                &self.kwic_query
            },
            self.selected_corpus
                .and_then(|i| self.corpora.get(i).map(|c| c.name.as_str()))
                .unwrap_or("-"),
            if self.kwic_sort.is_empty() {
                "none"
            } else {
                &self.kwic_sort
            },
            self.kwic_context,
            self.kwic_limit,
            self.kwic_regex
        );
        f.render_widget(
            Paragraph::new(header).style(Style::default().fg(self.theme.muted)),
            rows[0],
        );

        let table_rows: Vec<Row> = self
            .kwic_hits
            .iter()
            .map(|h| {
                Row::new(vec![
                    Cell::from(h.left.join(" ")),
                    Cell::from(Span::styled(h.match_text.clone(), self.theme.match_style())),
                    Cell::from(h.right.join(" ")),
                ])
            })
            .collect();
        let table = Table::new(
            table_rows,
            [
                Constraint::Percentage(40),
                Constraint::Length(16),
                Constraint::Percentage(40),
            ],
        )
        .header(Row::new(vec!["Left", "Node", "Right"]).style(self.theme.title()))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" Hits ({}) ", self.kwic_hits.len()))
                .style(Style::default().fg(self.theme.fg).bg(self.theme.bg)),
        )
        .row_highlight_style(self.theme.row_selected())
        .highlight_symbol("▸ ");
        f.render_stateful_widget(table, rows[1], &mut self.kwic_table);

        f.render_widget(
            Paragraph::new(self.kwic_detail.as_str())
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Context ")
                        .style(Style::default().fg(self.theme.fg).bg(self.theme.bg)),
                )
                .wrap(Wrap { trim: true }),
            rows[2],
        );
    }

    fn draw_stats(&self, f: &mut Frame<'_>, area: Rect) {
        let text = vec![
            Line::from("Frequency · Collocations · Keyness"),
            Line::from(""),
            Line::from(Span::styled(
                "Not yet available in the CLI (Stage 2).",
                Style::default().fg(self.theme.muted),
            )),
            Line::from(""),
            Line::from("When `glossa freq`, `glossa colloc`, and `glossa keyness` ship,"),
            Line::from("this tab will call the same in-process handlers as the CLI."),
        ];
        f.render_widget(
            Paragraph::new(text)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Statistics (stub) ")
                        .style(Style::default().fg(self.theme.fg).bg(self.theme.bg)),
                )
                .wrap(Wrap { trim: true }),
            area,
        );
    }

    fn draw_session(&self, f: &mut Frame<'_>, area: Rect) {
        let tail: Vec<Line> = self
            .session_lines
            .iter()
            .rev()
            .take(200)
            .rev()
            .map(|l| Line::from(l.as_str()))
            .collect();
        let body = if tail.is_empty() {
            vec![Line::from(
                "(empty — commands append to .glossa/session.jsonl)",
            )]
        } else {
            tail
        };
        f.render_widget(
            Paragraph::new(body)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Session log ")
                        .style(Style::default().fg(self.theme.fg).bg(self.theme.bg)),
                )
                .wrap(Wrap { trim: true }),
            area,
        );
    }
}

pub fn run_loop(
    terminal: &mut Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    let tick = Duration::from_millis(100);
    let mut last_tick = Instant::now();
    loop {
        app.poll_background();
        terminal.draw(|f| app.draw(f))?;
        if app.should_quit {
            break;
        }
        let timeout = tick.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                app.on_key(key);
            }
        }
        if last_tick.elapsed() >= tick {
            last_tick = Instant::now();
        }
    }
    Ok(())
}

pub fn run_terminal(config: RunConfig) -> Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(io::stdout(), crossterm::terminal::EnterAlternateScreen,)?;
    let mut terminal = Terminal::new(ratatui::backend::CrosstermBackend::new(io::stdout()))?;
    let mut app = App::new(config);
    let result = run_loop(&mut terminal, &mut app);
    crossterm::execute!(io::stdout(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;
    result
}

fn escape_csv(s: &str) -> String {
    s.replace('"', "\"\"")
}
