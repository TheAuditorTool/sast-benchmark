//! Status command implementation

use anyhow::Result;
use colored::Colorize;

use jobqueue_core::{JobFilter, JobState, JobStore, QueueName};
use jobqueue_db::SqliteStore;

pub async fn run(database: &str, json_output: bool) -> Result<()> {
    let store = SqliteStore::new(database)?;

    // Get counts for each state
    let pending = store.count(JobFilter::new().state(JobState::Pending)).await?;
    let running = store.count(JobFilter::new().state(JobState::Running)).await?;
    let completed = store.count(JobFilter::new().state(JobState::Completed)).await?;
    let failed = store.count(JobFilter::new().state(JobState::Failed)).await?;

    let total = pending + running + completed + failed;

    if json_output {
        println!("{}", serde_json::json!({
            "total": total,
            "pending": pending,
            "running": running,
            "completed": completed,
            "failed": failed,
            "database": database,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        }));
    } else {
        println!("{}", "JobQueue Status".green().bold());
        println!("{}", "═".repeat(40));
        println!();
        println!("  Database:   {}", database.cyan());
        println!();
        println!("  {} Total Jobs", total.to_string().white().bold());
        println!("  {} Pending", format!("{:>6}", pending).yellow());
        println!("  {} Running", format!("{:>6}", running).blue());
        println!("  {} Completed", format!("{:>6}", completed).green());
        println!("  {} Failed", format!("{:>6}", failed).red());
        println!();

        // Show error rate
        if total > 0 {
            let error_rate = (failed as f64 / total as f64) * 100.0;
            let rate_colored = if error_rate > 10.0 {
                format!("{:.1}%", error_rate).red()
            } else if error_rate > 5.0 {
                format!("{:.1}%", error_rate).yellow()
            } else {
                format!("{:.1}%", error_rate).green()
            };
            println!("  Error Rate: {}", rate_colored);
        }

        println!();
        println!("  Time: {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"));
    }

    Ok(())
}
