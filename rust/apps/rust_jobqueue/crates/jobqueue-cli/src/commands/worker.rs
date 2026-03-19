//! Worker command implementation

use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use colored::Colorize;
use tokio::signal;

use jobqueue_core::{Job, JobExecutor, QueueName, WorkerId};
use jobqueue_db::SqliteStore;
use jobqueue_worker::{Worker, WorkerConfig};

pub async fn run(
    database: &str,
    queues: &[&str],
    concurrency: usize,
    name: Option<String>,
) -> Result<()> {
    let worker_id = name
        .map(WorkerId::new)
        .unwrap_or_else(WorkerId::generate);

    println!("{}", "Starting JobQueue Worker...".green().bold());
    println!("  Worker ID: {}", worker_id.to_string().cyan());
    println!("  Database: {}", database.cyan());
    println!("  Queues: {}", queues.join(", ").cyan());
    println!("  Concurrency: {}", concurrency.to_string().cyan());

    // Open database
    let store = SqliteStore::new(database)?;

    // Configure worker
    let config = WorkerConfig {
        worker_id,
        queues: queues.iter().map(|q| QueueName::new(*q)).collect(),
        concurrency,
        poll_interval: Duration::from_millis(100),
        graceful_shutdown: true,
        ..Default::default()
    };

    // Create worker
    let worker = Worker::new(store, config);

    // Register built-in handlers
    register_handlers(&worker).await;

    println!("{}", "Worker started. Press Ctrl+C to stop.".green());

    // Run worker with graceful shutdown
    let handle = worker.handle();

    tokio::select! {
        result = worker.start() => {
            result?;
        }
        _ = signal::ctrl_c() => {
            println!("\n{}", "Shutting down worker...".yellow());
            handle.shutdown().await;
        }
    }

    println!("{}", "Worker stopped.".green());
    Ok(())
}

async fn register_handlers(worker: &Worker<SqliteStore>) {
    // Echo handler (for testing)
    worker.register_fn("echo", |job: &Job| async move {
        let data = job.payload.data.clone();
        Ok(serde_json::json!({
            "echo": data,
            "processed_at": chrono::Utc::now().to_rfc3339()
        }))
    }).await;

    // Sleep handler (for testing)
    worker.register_fn("sleep", |job: &Job| async move {
        let duration: u64 = job.payload.data
            .get("seconds")
            .and_then(|v| v.as_u64())
            .unwrap_or(1);

        tokio::time::sleep(Duration::from_secs(duration)).await;

        Ok(serde_json::json!({
            "slept_for_seconds": duration
        }))
    }).await;

    // Fail handler (for testing retries)
    worker.register_fn("fail", |_job: &Job| async move {
        Err(jobqueue_core::JobQueueError::Internal(
            "Intentional failure for testing".into()
        ))
    }).await;

    println!("  Registered handlers: {}", "echo, sleep, fail".cyan());
}
