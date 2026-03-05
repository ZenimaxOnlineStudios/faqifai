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
#[command(name = "faqifai", about = "AI-powered FAQ generator for codebases")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Maximum number of concurrent AI sessions
    #[arg(long, default_value_t = 4, global = true)]
    concurrency: usize,

    /// Model to use for answering questions
    #[arg(long, default_value = "claude-sonnet-4.6", global = true)]
    model: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate or regenerate FAQ answers
    Run {
        /// Force regeneration of all answers, ignoring TTL and hash state
        #[arg(long)]
        force: bool,

        /// Directory to scan for .faq files (defaults to current directory).
        /// The codebase root is always the current working directory.
        #[arg(short, long)]
        path: Option<std::path::PathBuf>,
    },
    /// Show status of all FAQ questions (fresh/stale/never-generated)
    Status {
        /// Directory to scan for .faq files (defaults to current directory)
        #[arg(short, long)]
        path: Option<std::path::PathBuf>,
    },
    /// List all known questions and their output files (machine-readable)
    List {
        /// Directory to scan for .faq files (defaults to current directory)
        #[arg(short, long)]
        path: Option<std::path::PathBuf>,

        /// Output format
        #[arg(long, default_value = "text")]
        format: OutputFormat,
    },
    /// Get the answer to a specific question by exact text or substring match
    Get {
        /// Question text or substring to match
        query: String,

        /// Directory to scan for .faq files (defaults to current directory)
        #[arg(short, long)]
        path: Option<std::path::PathBuf>,

        /// Print only the raw answer body with no decoration
        #[arg(long)]
        raw: bool,
    },
    /// Search across all generated answers for a keyword or pattern
    Search {
        /// Search pattern (substring or regex)
        pattern: String,

        /// Directory to scan for .faq files (defaults to current directory)
        #[arg(short, long)]
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
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    let root = std::env::current_dir()?;

    match cli.command {
        Commands::Run { force, path } => {
            let scan_path = path.unwrap_or_else(|| root.clone());
            orchestrator::run(&root, &scan_path, cli.concurrency, force, &cli.model).await?;
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
