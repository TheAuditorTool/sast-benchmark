//! Job command implementations

use anyhow::Result;
use colored::Colorize;

use jobqueue_core::{Job, JobFilter, JobId, JobState, JobStore, Priority, QueueName};
use jobqueue_db::SqliteStore;

use crate::JobCommands;

pub async fn run(database: &str, action: &JobCommands, json_output: bool) -> Result<()> {
    let store = SqliteStore::new(database)?;

    match action {
        JobCommands::Create {
            job_type,
            data,
            queue,
            priority,
            tags,
            schedule,
        } => {
            create_job(&store, job_type, data, queue, *priority, tags.as_deref(), schedule.as_deref(), json_output).await
        }

        JobCommands::List {
            queue,
            state,
            job_type,
            limit,
            verbose,
        } => {
            list_jobs(&store, queue.as_deref(), state.as_deref(), job_type.as_deref(), *limit, *verbose, json_output).await
        }

        JobCommands::Get { id } => {
            get_job(&store, id, json_output).await
        }

        JobCommands::Cancel { id, force } => {
            cancel_job(&store, id, *force, json_output).await
        }

        JobCommands::Retry { id } => {
            retry_job(&store, id, json_output).await
        }

        JobCommands::Search { query, limit } => {
            search_jobs(&store, query, *limit, json_output).await
        }

        JobCommands::Watch { queue, interval } => {
            watch_jobs(&store, queue.as_deref(), *interval).await
        }
    }
}

async fn create_job(
    store: &SqliteStore,
    job_type: &str,
    data: &str,
    queue: &str,
    priority: i32,
    tags: Option<&str>,
    schedule: Option<&str>,
    json_output: bool,
) -> Result<()> {
    let data: serde_json::Value = serde_json::from_str(data)?;

    let mut builder = Job::builder(job_type)
        .data_raw(data)
        .queue(queue)
        .priority(Priority::from_value(priority));

    if let Some(tags_str) = tags {
        for tag in tags_str.split(',') {
            builder = builder.tag(tag.trim());
        }
    }

    if let Some(schedule_str) = schedule {
        let scheduled_at = chrono::DateTime::parse_from_rfc3339(schedule_str)?
            .with_timezone(&chrono::Utc);
        builder = builder.schedule_at(scheduled_at);
    }

    let job = builder.build();

    store.save(&job).await?;

    if json_output {
        println!("{}", serde_json::json!({
            "id": job.id.to_string(),
            "status": "created"
        }));
    } else {
        println!("{} Job created: {}", "✓".green(), job.id.to_string().cyan());
    }

    Ok(())
}

async fn list_jobs(
    store: &SqliteStore,
    queue: Option<&str>,
    state: Option<&str>,
    job_type: Option<&str>,
    limit: usize,
    verbose: bool,
    json_output: bool,
) -> Result<()> {
    let mut filter = JobFilter::new().limit(limit);

    if let Some(q) = queue {
        filter = filter.queue(q);
    }

    if let Some(s) = state {
        let job_state = match s {
            "pending" => JobState::Pending,
            "running" => JobState::Running,
            "completed" => JobState::Completed,
            "failed" => JobState::Failed,
            _ => return Err(anyhow::anyhow!("Invalid state: {}", s)),
        };
        filter = filter.state(job_state);
    }

    if let Some(jt) = job_type {
        filter = filter.job_type(jt);
    }

    let jobs = store.list(filter).await?;

    if json_output {
        let output: Vec<_> = jobs.iter().map(|j| serde_json::json!({
            "id": j.id.to_string(),
            "type": j.payload.job_type,
            "queue": j.queue.to_string(),
            "state": format!("{:?}", j.state).to_lowercase(),
            "priority": j.priority.value(),
            "attempt": j.attempt,
            "created_at": j.created_at.to_rfc3339(),
        })).collect();
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        if jobs.is_empty() {
            println!("{}", "No jobs found.".yellow());
            return Ok(());
        }

        println!("{} jobs found:\n", jobs.len().to_string().green());

        for job in jobs {
            let state_colored = match job.state {
                JobState::Pending => "pending".yellow(),
                JobState::Running => "running".blue(),
                JobState::Completed => "completed".green(),
                JobState::Failed => "failed".red(),
                _ => format!("{:?}", job.state).normal(),
            };

            println!(
                "  {} [{}] {} ({})",
                job.id.to_string().cyan(),
                state_colored,
                job.payload.job_type,
                job.queue
            );

            if verbose {
                println!("    Priority: {}, Attempt: {}", job.priority.value(), job.attempt);
                println!("    Created: {}", job.created_at);
                if let Some(error) = &job.error {
                    println!("    Error: {}", error.red());
                }
            }
        }
    }

    Ok(())
}

async fn get_job(store: &SqliteStore, id: &str, json_output: bool) -> Result<()> {
    let job_id = JobId::from(id.to_string());
    let job = store.get(&job_id).await?
        .ok_or_else(|| anyhow::anyhow!("Job not found: {}", id))?;

    if json_output {
        println!("{}", serde_json::to_string_pretty(&serde_json::json!({
            "id": job.id.to_string(),
            "type": job.payload.job_type,
            "queue": job.queue.to_string(),
            "state": format!("{:?}", job.state).to_lowercase(),
            "priority": job.priority.value(),
            "attempt": job.attempt,
            "data": job.payload.data,
            "created_at": job.created_at.to_rfc3339(),
            "updated_at": job.updated_at.to_rfc3339(),
            "error": job.error,
            "result": job.result,
        }))?);
    } else {
        println!("{}", "Job Details".green().bold());
        println!("  ID:       {}", job.id.to_string().cyan());
        println!("  Type:     {}", job.payload.job_type);
        println!("  Queue:    {}", job.queue);
        println!("  State:    {:?}", job.state);
        println!("  Priority: {}", job.priority.value());
        println!("  Attempt:  {}", job.attempt);
        println!("  Created:  {}", job.created_at);
        println!("  Updated:  {}", job.updated_at);
        println!("  Data:     {}", serde_json::to_string(&job.payload.data)?);
        if let Some(error) = &job.error {
            println!("  Error:    {}", error.red());
        }
        if let Some(result) = &job.result {
            println!("  Result:   {}", serde_json::to_string(result)?);
        }
    }

    Ok(())
}

async fn cancel_job(store: &SqliteStore, id: &str, _force: bool, json_output: bool) -> Result<()> {
    let job_id = JobId::from(id.to_string());
    store.update_state(&job_id, JobState::Cancelled).await?;

    if json_output {
        println!("{}", serde_json::json!({"id": id, "status": "cancelled"}));
    } else {
        println!("{} Job cancelled: {}", "✓".green(), id.cyan());
    }

    Ok(())
}

async fn retry_job(store: &SqliteStore, id: &str, json_output: bool) -> Result<()> {
    let job_id = JobId::from(id.to_string());
    store.update_state(&job_id, JobState::Pending).await?;

    if json_output {
        println!("{}", serde_json::json!({"id": id, "status": "retrying"}));
    } else {
        println!("{} Job queued for retry: {}", "✓".green(), id.cyan());
    }

    Ok(())
}

async fn search_jobs(store: &SqliteStore, query: &str, limit: usize, json_output: bool) -> Result<()> {
    // VULNERABILITY: Search query passed to SQL without sanitization
    let jobs = store.search_jobs(query, limit).await?;

    if json_output {
        let output: Vec<_> = jobs.iter().map(|j| serde_json::json!({
            "id": j.id.to_string(),
            "type": j.payload.job_type,
            "state": format!("{:?}", j.state).to_lowercase(),
        })).collect();
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        println!("Found {} jobs matching '{}':", jobs.len(), query.cyan());
        for job in jobs {
            println!("  {} - {} ({:?})", job.id, job.payload.job_type, job.state);
        }
    }

    Ok(())
}

async fn watch_jobs(store: &SqliteStore, queue: Option<&str>, interval: u64) -> Result<()> {
    println!("{}", "Watching jobs (Ctrl+C to stop)...".green());

    loop {
        // Clear screen
        print!("\x1B[2J\x1B[1;1H");

        let mut filter = JobFilter::new().limit(20);
        if let Some(q) = queue {
            filter = filter.queue(q);
        }

        let jobs = store.list(filter).await?;

        println!("{} - {} jobs", chrono::Utc::now().format("%H:%M:%S"), jobs.len());
        println!("{}", "-".repeat(60));

        for job in jobs {
            let state_colored = match job.state {
                JobState::Running => "▶".blue(),
                JobState::Pending => "○".yellow(),
                JobState::Completed => "✓".green(),
                JobState::Failed => "✗".red(),
                _ => "?".normal(),
            };

            println!(
                "{} {} {} {}",
                state_colored,
                job.id.to_string().chars().take(8).collect::<String>(),
                job.payload.job_type,
                job.queue
            );
        }

        tokio::time::sleep(std::time::Duration::from_secs(interval)).await;
    }
}
