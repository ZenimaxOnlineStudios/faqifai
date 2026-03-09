mod ai;
mod codebase;
mod config;
mod copilot;
mod discovery;
mod eval;
mod orchestrator;
mod output;
mod state;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "faqifai",
    about = "AI-powered FAQ generator for codebases.",
    long_about = "Discovers .faq files in a repository, uses GitHub Copilot to research the codebase, \
and writes terse sourced markdown answers. Answers are cached and only regenerated when source \
files change or the TTL expires."
)]
struct Cli {
    /// Enable verbose debug logging (shows JSON-RPC events, tool calls, session lifecycle)
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate or regenerate FAQ answers
    Run {
        /// Force regeneration of all answers, ignoring TTL and source hashes
        #[arg(long)]
        force: bool,

        /// Directory to scan for .faq files (defaults to CWD).
        /// The codebase root is always the directory where faqifai is invoked.
        #[arg(short, long, value_name = "DIR")]
        path: Option<std::path::PathBuf>,

        /// Number of questions to answer in parallel
        #[arg(long, default_value_t = 4, value_name = "N")]
        concurrency: usize,

        /// Copilot model to use for answering questions
        #[arg(long, default_value = "claude-sonnet-4.6", value_name = "MODEL")]
        model: String,
    },
    /// Show staleness status of all questions (fresh / TTL expired / sources changed / never generated)
    Status {
        /// Directory to scan for .faq files (defaults to CWD)
        #[arg(short, long, value_name = "DIR")]
        path: Option<std::path::PathBuf>,
    },
    /// List all known questions and their output files
    List {
        /// Directory to scan for .faq files (defaults to CWD)
        #[arg(short, long, value_name = "DIR")]
        path: Option<std::path::PathBuf>,

        /// Output format
        #[arg(long, default_value = "text")]
        format: OutputFormat,
    },
    /// Print the answer to a question (by substring match)
    Get {
        /// Question text or substring to match
        query: String,

        /// Directory to scan for .faq files (defaults to CWD)
        #[arg(short, long, value_name = "DIR")]
        path: Option<std::path::PathBuf>,

        /// Print only the raw answer body with no decoration
        #[arg(long)]
        raw: bool,
    },
    /// Search across all generated answers for a keyword or pattern
    Search {
        /// Search pattern (substring or regex)
        pattern: String,

        /// Directory to scan for .faq files (defaults to CWD)
        #[arg(short, long, value_name = "DIR")]
        path: Option<std::path::PathBuf>,

        /// Output format
        #[arg(long, default_value = "text")]
        format: OutputFormat,
    },
}

#[derive(Clone, Debug, clap::ValueEnum)]
enum OutputFormat {
    Text,
    Json,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Verbose flag enables DEBUG; default shows only WARN+
    let level = if cli.verbose { "debug" } else { "warn" };
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(level)),
        )
        .with_writer(std::io::stderr)
        .init();

    let cli = Cli::parse();
    let root = std::env::current_dir()?;

    match cli.command {
        Commands::Run { force, path, concurrency, model } => {
            let scan_path = path.unwrap_or_else(|| root.clone());
            orchestrator::run(&root, &scan_path, concurrency, force, &model).await?;
        }
        Commands::Status { path } => {
            let scan_path = path.unwrap_or_else(|| root.clone());
            orchestrator::status(&root, &scan_path)?;
        }
        Commands::List { path, format } => {
            let scan_path = path.unwrap_or_else(|| root.clone());
            orchestrator::list(&root, &scan_path, matches!(format, OutputFormat::Json))?;
        }
        Commands::Get { query, path, raw } => {
            let scan_path = path.unwrap_or_else(|| root.clone());
            orchestrator::get(&root, &scan_path, &query, raw)?;
        }
        Commands::Search { pattern, path, format } => {
            let scan_path = path.unwrap_or_else(|| root.clone());
            orchestrator::search(&root, &scan_path, &pattern, matches!(format, OutputFormat::Json))?;
        }
    }

    Ok(())
}
