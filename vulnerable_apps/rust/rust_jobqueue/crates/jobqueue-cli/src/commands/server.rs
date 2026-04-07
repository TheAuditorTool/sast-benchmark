//! Server command implementation

use anyhow::Result;
use colored::Colorize;

use jobqueue_api::{ApiConfig, ApiServer};
use jobqueue_db::SqliteStore;

pub async fn run(
    database: &str,
    host: &str,
    port: u16,
    api_key: Option<String>,
) -> Result<()> {
    println!("{}", "Starting JobQueue API Server...".green().bold());
    println!("  Database: {}", database.cyan());
    println!("  Listening: {}:{}", host.cyan(), port.to_string().cyan());

    if api_key.is_some() {
        println!("  Auth: {}", "API Key enabled".yellow());
    } else {
        println!("  Auth: {}", "None (open access)".red());
    }

    // Open database
    let store = SqliteStore::new(database)?;

    // Configure API
    let config = ApiConfig {
        host: host.to_string(),
        port,
        api_key,
        cors_enabled: true,
        ..Default::default()
    };

    // Start server
    let server = ApiServer::new(store, config);
    server.run().await?;

    Ok(())
}
