//! Database connection pool management.
//!
//! TAINT FLOW: Environment variables (SOURCE) -> database URL -> pool creation

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::str::FromStr;
use std::time::Duration;

/// Type alias for the database pool
pub type DbPool = SqlitePool;

/// Create a database connection pool from database URL.
///
/// TAINT FLOW: database_url (from env) -> SqliteConnectOptions -> pool
pub async fn create_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    // TAINT SOURCE: database_url comes from environment variable
    println!("Connecting to database: {}", database_url);

    // Parse connection options from potentially tainted URL
    let options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
        .busy_timeout(Duration::from_secs(30));

    // Create pool with configured options
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .min_connections(1)
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(300))
        .connect_with(options)
        .await?;

    println!("Database pool created successfully");

    Ok(pool)
}

/// Test database connection
pub async fn test_connection(pool: &DbPool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1").fetch_one(pool).await?;
    Ok(())
}

/// Get pool statistics for monitoring
pub fn get_pool_stats(pool: &DbPool) -> PoolStats {
    PoolStats {
        size: pool.size(),
        idle: pool.num_idle(),
    }
}

#[derive(Debug, serde::Serialize)]
pub struct PoolStats {
    pub size: u32,
    pub idle: usize,
}
