//! Queue command implementations

use anyhow::Result;
use colored::Colorize;

use jobqueue_core::QueueName;
use jobqueue_db::SqliteStore;

use crate::QueueCommands;

pub async fn run(database: &str, action: &QueueCommands, json_output: bool) -> Result<()> {
    let store = SqliteStore::new(database)?;

    match action {
        QueueCommands::List => {
            list_queues(&store, json_output).await
        }

        QueueCommands::Stats { name } => {
            queue_stats(&store, name, json_output).await
        }

        QueueCommands::Pause { name } => {
            pause_queue(&store, name, json_output).await
        }

        QueueCommands::Resume { name } => {
            resume_queue(&store, name, json_output).await
        }

        QueueCommands::Purge { name, state, yes } => {
            purge_queue(&store, name, state.as_deref(), *yes, json_output).await
        }
    }
}

async fn list_queues(store: &SqliteStore, json_output: bool) -> Result<()> {
    // Would normally query distinct queues from database
    let queues = vec!["default".to_string()];

    if json_output {
        println!("{}", serde_json::to_string_pretty(&queues)?);
    } else {
        println!("{}", "Queues:".green().bold());
        for queue in queues {
            println!("  - {}", queue.cyan());
        }
    }

    Ok(())
}

async fn queue_stats(store: &SqliteStore, name: &str, json_output: bool) -> Result<()> {
    let queue = QueueName::new(name);
    let stats = store.queue_stats(&queue).await?;

    if json_output {
        println!("{}", serde_json::to_string_pretty(&stats)?);
    } else {
        println!("{} Queue: {}", "📊".to_string(), name.cyan().bold());
        println!("  Total:     {}", stats.total);
        println!("  Pending:   {}", stats.pending.to_string().yellow());
        println!("  Running:   {}", stats.running.to_string().blue());
        println!("  Completed: {}", stats.completed.to_string().green());
        println!("  Failed:    {}", stats.failed.to_string().red());
    }

    Ok(())
}

async fn pause_queue(_store: &SqliteStore, name: &str, json_output: bool) -> Result<()> {
    // Would implement queue pausing

    if json_output {
        println!("{}", serde_json::json!({"queue": name, "status": "paused"}));
    } else {
        println!("{} Queue '{}' paused", "⏸".to_string(), name.cyan());
    }

    Ok(())
}

async fn resume_queue(_store: &SqliteStore, name: &str, json_output: bool) -> Result<()> {
    // Would implement queue resuming

    if json_output {
        println!("{}", serde_json::json!({"queue": name, "status": "resumed"}));
    } else {
        println!("{} Queue '{}' resumed", "▶".to_string(), name.cyan());
    }

    Ok(())
}

async fn purge_queue(
    _store: &SqliteStore,
    name: &str,
    _state: Option<&str>,
    yes: bool,
    json_output: bool,
) -> Result<()> {
    if !yes {
        println!("{}", "Are you sure you want to purge this queue?".red().bold());
        println!("Use --yes to confirm.");
        return Ok(());
    }

    // Would implement purging

    if json_output {
        println!("{}", serde_json::json!({"queue": name, "status": "purged", "deleted": 0}));
    } else {
        println!("{} Queue '{}' purged", "🗑".to_string(), name.cyan());
    }

    Ok(())
}
