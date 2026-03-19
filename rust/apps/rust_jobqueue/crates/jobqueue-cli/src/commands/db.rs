//! Database command implementations
//!
//! VULNERABILITY: execute_sql allows arbitrary SQL execution

use anyhow::Result;
use colored::Colorize;

use jobqueue_db::SqliteStore;

use crate::DbCommands;

pub async fn run(database: &str, action: &DbCommands) -> Result<()> {
    match action {
        DbCommands::Migrate => {
            migrate(database).await
        }

        DbCommands::Backup { output } => {
            backup(database, output.as_deref()).await
        }

        DbCommands::Restore { path, yes } => {
            restore(database, path, *yes).await
        }

        DbCommands::Cleanup { days, dry_run } => {
            cleanup(database, *days, *dry_run).await
        }

        DbCommands::Sql { query } => {
            execute_sql(database, query).await
        }
    }
}

async fn migrate(database: &str) -> Result<()> {
    println!("{}", "Running database migrations...".green());

    // Opening the store runs migrations automatically
    let _store = SqliteStore::new(database)?;

    println!("{} Migrations complete.", "✓".green());
    Ok(())
}

async fn backup(database: &str, output: Option<&str>) -> Result<()> {
    let backup_path = output.unwrap_or("jobqueue_backup.db");

    println!("Creating backup: {} -> {}", database.cyan(), backup_path.cyan());

    std::fs::copy(database, backup_path)?;

    println!("{} Backup created: {}", "✓".green(), backup_path);
    Ok(())
}

async fn restore(database: &str, backup_path: &str, yes: bool) -> Result<()> {
    if !yes {
        println!("{}", "WARNING: This will overwrite the current database!".red().bold());
        println!("Use --yes to confirm.");
        return Ok(());
    }

    println!("Restoring from backup: {} -> {}", backup_path.cyan(), database.cyan());

    std::fs::copy(backup_path, database)?;

    println!("{} Database restored.", "✓".green());
    Ok(())
}

async fn cleanup(database: &str, days: i64, dry_run: bool) -> Result<()> {
    let store = SqliteStore::new(database)?;

    if dry_run {
        println!("DRY RUN - No changes will be made");
    }

    println!("Cleaning up jobs older than {} days...", days);

    let deleted = store.cleanup(chrono::Duration::days(days)).await?;

    if dry_run {
        println!("Would delete {} jobs", deleted);
    } else {
        println!("{} Deleted {} jobs", "✓".green(), deleted);
    }

    Ok(())
}

/// Execute raw SQL
///
/// VULNERABILITY: Arbitrary SQL execution without authorization
async fn execute_sql(database: &str, query: &str) -> Result<()> {
    println!("{}", "WARNING: Executing raw SQL!".red().bold());
    println!("Query: {}", query.yellow());

    let store = SqliteStore::new(database)?;

    // TAINT SINK: Arbitrary SQL execution
    let rows = store.execute_raw(query).await?;

    println!("{} Rows affected: {}", "✓".green(), rows);

    Ok(())
}
