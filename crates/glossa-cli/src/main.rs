mod brand;
mod commands;
mod output;

use anyhow::{Context, Result};
use chrono::Utc;
use clap::{Parser, Subcommand};
use glossa_core::{resolve_output_mode, OutputMode};
use glossa_workspace::{SessionEvent, Workspace};
use serde_json::json;
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

#[derive(Debug, Parser)]
#[command(
    name = "glossa",
    about = "Offline-first linguistics CLI/TUI",
    disable_version_flag = true
)]
struct Cli {
    /// Print version (JSON envelope with --json)
    #[arg(short = 'V', long = "version", global = true)]
    version: bool,

    /// Emit machine-readable JSON result envelope
    #[arg(long, global = true)]
    json: bool,

    /// Linear plain output (screen-reader friendly)
    #[arg(long, global = true)]
    plain: bool,

    /// Disable ANSI colours
    #[arg(long, global = true)]
    no_color: bool,

    /// Workspace directory (contains glossa.toml)
    #[arg(long, global = true, env = "GLOSSA_WORKSPACE")]
    workspace: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Create a new Glossa workspace
    Init {
        /// Directory to initialise (default: current directory)
        dir: Option<PathBuf>,
        /// Workspace name (defaults to directory name)
        #[arg(long)]
        name: Option<String>,
    },
    /// Corpus import and management
    Corpus {
        #[command(subcommand)]
        action: CorpusCmd,
    },
    /// Keyword in context concordance
    Kwic {
        /// Query string (plain word or regex with --regex)
        query: String,
        /// Corpus name (default: first / only corpus)
        #[arg(short, long)]
        corpus: Option<String>,
        /// Treat query as a regular expression over word forms
        #[arg(long)]
        regex: bool,
        /// Sort key: L1, R1, or R2
        #[arg(long, default_value = "")]
        sort: String,
        /// Tokens of context on each side
        #[arg(long, default_value_t = 5)]
        context: usize,
        /// Maximum hits to return
        #[arg(long, default_value_t = 100)]
        limit: usize,
    },
    /// Show Glossa branding and project pointers
    About,
}

#[derive(Debug, Subcommand)]
enum CorpusCmd {
    /// Add documents from a file or directory
    Add {
        path: PathBuf,
        /// Corpus name
        #[arg(long)]
        name: Option<String>,
        /// Optional metadata CSV keyed by filename
        #[arg(long)]
        meta: Option<PathBuf>,
    },
    /// List corpora
    Ls,
    /// Show corpus info
    Info { name: String },
    /// Remove a corpus
    Rm { name: String },
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(err) => {
            eprintln!("error: {err:#}");
            ExitCode::from(1)
        }
    }
}

fn run() -> Result<ExitCode> {
    let cli = Cli::parse();
    let mode = resolve_output_mode(cli.json, cli.plain);
    let no_color = cli.no_color || std::env::var_os("NO_COLOR").is_some();
    let started = Instant::now();
    let started_at = Utc::now();

    if cli.version {
        let data = json!({
            "name": "glossa",
            "version": glossa_core::GLOSSA_VERSION,
        });
        output::emit(
            output::EmitArgs {
                mode,
                command: "version",
                params: Default::default(),
                started_at,
                duration: started.elapsed(),
                corpus_hash: None,
                data: &data,
            },
            |d| {
                println!("glossa {}", d["version"].as_str().unwrap_or("?"));
            },
        )?;
        if let Ok(ws) = open_workspace_optional(&cli.workspace) {
            let _ = ws.session_log().append(&SessionEvent {
                ts: started_at,
                command: "version".into(),
                args: json!({}),
                ok: true,
                error: None,
                duration_ms: Some(started.elapsed().as_millis() as u64),
            });
        }
        return Ok(ExitCode::SUCCESS);
    }

    // Bare `glossa` would open TUI in Stage 4; for now show a short banner.
    let Some(command) = cli.command else {
        let data = json!({
            "name": "glossa",
            "version": glossa_core::GLOSSA_VERSION,
            "message": "TUI opens here in Stage 4; use --help for commands",
        });
        output::emit(
            output::EmitArgs {
                mode,
                command: "glossa",
                params: Default::default(),
                started_at,
                duration: started.elapsed(),
                corpus_hash: None,
                data: &data,
            },
            |d| {
                if brand::banner_env_enabled() {
                    brand::print_banner(brand::use_color(no_color));
                    println!();
                }
                println!("glossa {}", d["version"].as_str().unwrap_or("?"));
                println!("{}", d["message"].as_str().unwrap_or(""));
            },
        )?;
        return Ok(ExitCode::SUCCESS);
    };

    let cmd_name = command_name(&command);
    let mut log_ok = true;
    let mut log_err: Option<String> = None;
    let result = dispatch(&cli.workspace, mode, no_color, command);

    if let Err(ref e) = result {
        log_ok = false;
        log_err = Some(format!("{e:#}"));
    }

    // Best-effort session log for every invocation that has a workspace.
    if let Ok(ws) = open_workspace_optional(&cli.workspace) {
        let _ = ws.session_log().append(&SessionEvent {
            ts: started_at,
            command: cmd_name.to_string(),
            args: json!({}),
            ok: log_ok,
            error: log_err,
            duration_ms: Some(started.elapsed().as_millis() as u64),
        });
    }

    result
}

fn command_name(cmd: &Commands) -> &'static str {
    match cmd {
        Commands::Init { .. } => "init",
        Commands::Corpus { action } => match action {
            CorpusCmd::Add { .. } => "corpus add",
            CorpusCmd::Ls => "corpus ls",
            CorpusCmd::Info { .. } => "corpus info",
            CorpusCmd::Rm { .. } => "corpus rm",
        },
        Commands::Kwic { .. } => "kwic",
        Commands::About => "about",
    }
}

fn open_workspace_optional(explicit: &Option<PathBuf>) -> Result<Workspace> {
    let start = explicit
        .clone()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    Workspace::open(&start).map_err(Into::into)
}

fn require_workspace(explicit: &Option<PathBuf>) -> Result<Workspace> {
    open_workspace_optional(explicit).context("workspace not found; run `glossa init` first")
}

fn dispatch(
    workspace: &Option<PathBuf>,
    mode: OutputMode,
    no_color: bool,
    command: Commands,
) -> Result<ExitCode> {
    match command {
        Commands::Init { dir, name } => {
            commands::init::run(dir, name, mode)?;
            Ok(ExitCode::SUCCESS)
        }
        Commands::Corpus { action } => {
            let ws = require_workspace(workspace)?;
            match action {
                CorpusCmd::Add { path, name, meta } => {
                    commands::corpus::add(&ws, path, name, meta, mode)?;
                }
                CorpusCmd::Ls => commands::corpus::ls(&ws, mode)?,
                CorpusCmd::Info { name } => commands::corpus::info(&ws, &name, mode)?,
                CorpusCmd::Rm { name } => commands::corpus::rm(&ws, &name, mode)?,
            }
            Ok(ExitCode::SUCCESS)
        }
        Commands::Kwic {
            query,
            corpus,
            regex,
            sort,
            context,
            limit,
        } => {
            let ws = require_workspace(workspace)?;
            commands::kwic::run(
                &ws,
                commands::kwic::KwicArgs {
                    query,
                    corpus,
                    regex,
                    sort,
                    context,
                    limit,
                    mode,
                },
            )?;
            Ok(ExitCode::SUCCESS)
        }
        Commands::About => {
            let data = json!({
                "name": "glossa",
                "version": glossa_core::GLOSSA_VERSION,
                "homepage": "https://glossa.tools",
                "repository": "https://github.com/glossa-tools/glossa",
                "tagline": "every linguist's research tool, in one fast terminal workspace",
            });
            let started_at = Utc::now();
            let started = Instant::now();
            output::emit(
                output::EmitArgs {
                    mode,
                    command: "about",
                    params: Default::default(),
                    started_at,
                    duration: started.elapsed(),
                    corpus_hash: None,
                    data: &data,
                },
                |d| {
                    brand::print_banner(brand::use_color(no_color));
                    println!();
                    println!("glossa {}", d["version"].as_str().unwrap_or("?"));
                    println!("{}", d["homepage"].as_str().unwrap_or(""));
                    println!("{}", d["repository"].as_str().unwrap_or(""));
                    println!("brand assets: brand/ (see brand/README.md)");
                },
            )?;
            Ok(ExitCode::SUCCESS)
        }
    }
}
