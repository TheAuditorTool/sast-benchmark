//! JobQueue CLI
//!
//! Command-line interface for managing the job queue.
//!
//! # Usage
//!
//! ```bash
//! # Start the server
//! jq server --port 8080
//!
//! # Start a worker
//! jq worker --queues default,emails
//!
//! # Create a job
//! jq job create --type email --data '{"to": "test@example.com"}'
//!
//! # List jobs
//! jq job list --queue default --state pending
//!
//! # Get job details
//! jq job get <job_id>
//! ```

mod commands;

use clap::{Parser, Subcommand};
use colored::Colorize;

/// JobQueue CLI - Distributed job queue management
#[derive(Parser)]
#[command(name = "jq")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Database path
    #[arg(short, long, default_value = "jobqueue.db", env = "JQ_DATABASE")]
    database: String,

    /// Log level
    #[arg(short, long, default_value = "info", env = "JQ_LOG_LEVEL")]
    log_level: String,

    /// Output format (text, json)
    #[arg(short, long, default_value = "text")]
    format: OutputFormat,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
enum OutputFormat {
    Text,
    Json,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the API server
    Server {
        /// Port to listen on
        #[arg(short, long, default_value = "8080")]
        port: u16,

        /// Host to bind to
        #[arg(short = 'H', long, default_value = "0.0.0.0")]
        host: String,

        /// API key for authentication
        #[arg(short, long, env = "JQ_API_KEY")]
        api_key: Option<String>,
    },

    /// Start a worker process
    Worker {
        /// Queues to process (comma-separated)
        #[arg(short, long, default_value = "default")]
        queues: String,

        /// Number of concurrent jobs
        #[arg(short, long, default_value = "4")]
        concurrency: usize,

        /// Worker name/ID
        #[arg(short, long)]
        name: Option<String>,
    },

    /// Job management commands
    Job {
        #[command(subcommand)]
        action: JobCommands,
    },

    /// Queue management commands
    Queue {
        #[command(subcommand)]
        action: QueueCommands,
    },

    /// Database operations
    Db {
        #[command(subcommand)]
        action: DbCommands,
    },

    /// Show system status
    Status,

    /// Interactive shell
    Shell,
}

#[derive(Subcommand)]
enum JobCommands {
    /// Create a new job
    Create {
        /// Job type
        #[arg(short = 't', long)]
        job_type: String,

        /// Job data (JSON)
        #[arg(short, long)]
        data: String,

        /// Queue name
        #[arg(short, long, default_value = "default")]
        queue: String,

        /// Priority (1-100)
        #[arg(short, long, default_value = "5")]
        priority: i32,

        /// Tags (comma-separated)
        #[arg(long)]
        tags: Option<String>,

        /// Schedule for later (ISO 8601)
        #[arg(long)]
        schedule: Option<String>,
    },

    /// List jobs
    List {
        /// Filter by queue
        #[arg(short, long)]
        queue: Option<String>,

        /// Filter by state
        #[arg(short, long)]
        state: Option<String>,

        /// Filter by job type
        #[arg(short = 't', long)]
        job_type: Option<String>,

        /// Maximum results
        #[arg(short, long, default_value = "50")]
        limit: usize,

        /// Show detailed output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Get job details
    Get {
        /// Job ID
        id: String,
    },

    /// Cancel a job
    Cancel {
        /// Job ID
        id: String,

        /// Force cancel even if running
        #[arg(short, long)]
        force: bool,
    },

    /// Retry a failed job
    Retry {
        /// Job ID
        id: String,
    },

    /// Search jobs
    Search {
        /// Search query
        query: String,

        /// Maximum results
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },

    /// Watch jobs in real-time
    Watch {
        /// Filter by queue
        #[arg(short, long)]
        queue: Option<String>,

        /// Refresh interval (seconds)
        #[arg(short, long, default_value = "2")]
        interval: u64,
    },
}

#[derive(Subcommand)]
enum QueueCommands {
    /// List all queues
    List,

    /// Get queue statistics
    Stats {
        /// Queue name
        name: String,
    },

    /// Pause a queue
    Pause {
        /// Queue name
        name: String,
    },

    /// Resume a queue
    Resume {
        /// Queue name
        name: String,
    },

    /// Purge all jobs from a queue
    Purge {
        /// Queue name
        name: String,

        /// Only purge specific states
        #[arg(short, long)]
        state: Option<String>,

        /// Confirm without prompt
        #[arg(short, long)]
        yes: bool,
    },
}

#[derive(Subcommand)]
enum DbCommands {
    /// Run database migrations
    Migrate,

    /// Create a backup
    Backup {
        /// Backup file path
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Restore from backup
    Restore {
        /// Backup file path
        path: String,

        /// Confirm without prompt
        #[arg(short, long)]
        yes: bool,
    },

    /// Cleanup old jobs
    Cleanup {
        /// Delete jobs older than N days
        #[arg(short, long, default_value = "30")]
        days: i64,

        /// Dry run (show what would be deleted)
        #[arg(long)]
        dry_run: bool,
    },

    /// Execute raw SQL (DANGEROUS)
    Sql {
        /// SQL query to execute
        query: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(&cli.log_level)
        .init();

    // Run command
    match &cli.command {
        Commands::Server { port, host, api_key } => {
            commands::server::run(&cli.database, host, *port, api_key.clone()).await
        }

        Commands::Worker { queues, concurrency, name } => {
            let queue_list: Vec<&str> = queues.split(',').collect();
            commands::worker::run(&cli.database, &queue_list, *concurrency, name.clone()).await
        }

        Commands::Job { action } => {
            commands::job::run(&cli.database, action, cli.format == OutputFormat::Json).await
        }

        Commands::Queue { action } => {
            commands::queue::run(&cli.database, action, cli.format == OutputFormat::Json).await
        }

        Commands::Db { action } => {
            commands::db::run(&cli.database, action).await
        }

        Commands::Status => {
            commands::status::run(&cli.database, cli.format == OutputFormat::Json).await
        }

        Commands::Shell => {
            commands::shell::run(&cli.database).await
        }
    }
}
