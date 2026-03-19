//! Database migrations
//!
//! This module handles database schema migrations.

use rusqlite::Connection;
use crate::DbResult;

/// Migration version
const CURRENT_VERSION: i32 = 1;

/// Run all pending migrations
pub fn run_migrations(conn: &Connection) -> DbResult<()> {
    // Create migrations table if not exists
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_migrations (
            version INTEGER PRIMARY KEY,
            applied_at TEXT NOT NULL
        )",
        [],
    )?;

    let current: i32 = conn
        .query_row(
            "SELECT COALESCE(MAX(version), 0) FROM schema_migrations",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    if current < 1 {
        migrate_v1(conn)?;
    }

    Ok(())
}

/// Migration to version 1: Initial schema
fn migrate_v1(conn: &Connection) -> DbResult<()> {
    conn.execute_batch(
        "
        -- Main jobs table
        CREATE TABLE IF NOT EXISTS jobs (
            id TEXT PRIMARY KEY,
            queue TEXT NOT NULL,
            state TEXT NOT NULL DEFAULT 'pending',
            priority INTEGER NOT NULL DEFAULT 5,
            payload TEXT NOT NULL,
            retry_config TEXT NOT NULL,
            attempt INTEGER NOT NULL DEFAULT 0,
            worker_id TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            scheduled_at TEXT,
            started_at TEXT,
            finished_at TEXT,
            timeout_ms INTEGER,
            error TEXT,
            dependencies TEXT NOT NULL DEFAULT '[]',
            tags TEXT NOT NULL DEFAULT '[]',
            progress INTEGER,
            result TEXT
        );

        -- Indexes for common queries
        CREATE INDEX IF NOT EXISTS idx_jobs_queue_state ON jobs(queue, state);
        CREATE INDEX IF NOT EXISTS idx_jobs_state ON jobs(state);
        CREATE INDEX IF NOT EXISTS idx_jobs_priority ON jobs(priority DESC);
        CREATE INDEX IF NOT EXISTS idx_jobs_scheduled ON jobs(scheduled_at) WHERE state = 'scheduled';
        CREATE INDEX IF NOT EXISTS idx_jobs_worker ON jobs(worker_id) WHERE worker_id IS NOT NULL;
        CREATE INDEX IF NOT EXISTS idx_jobs_created ON jobs(created_at);

        -- Queue metadata table
        CREATE TABLE IF NOT EXISTS queues (
            name TEXT PRIMARY KEY,
            paused INTEGER NOT NULL DEFAULT 0,
            max_size INTEGER,
            rate_limit INTEGER,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        -- Workers table
        CREATE TABLE IF NOT EXISTS workers (
            id TEXT PRIMARY KEY,
            hostname TEXT NOT NULL,
            pid INTEGER NOT NULL,
            queues TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'active',
            last_heartbeat TEXT NOT NULL,
            started_at TEXT NOT NULL,
            jobs_processed INTEGER NOT NULL DEFAULT 0,
            jobs_failed INTEGER NOT NULL DEFAULT 0
        );

        -- Dead letter queue for failed jobs
        CREATE TABLE IF NOT EXISTS dead_letter_jobs (
            id TEXT PRIMARY KEY,
            original_job_id TEXT NOT NULL,
            queue TEXT NOT NULL,
            payload TEXT NOT NULL,
            error TEXT NOT NULL,
            attempts INTEGER NOT NULL,
            failed_at TEXT NOT NULL
        );

        -- Record migration
        INSERT INTO schema_migrations (version, applied_at)
        VALUES (1, datetime('now'));
        "
    )?;

    tracing::info!("Applied migration v1: Initial schema");
    Ok(())
}

/// Rollback a migration (dangerous!)
///
/// VULNERABILITY: No authorization check for rollback
pub fn rollback_migration(conn: &Connection, version: i32) -> DbResult<()> {
    match version {
        1 => {
            conn.execute_batch(
                "
                DROP TABLE IF EXISTS dead_letter_jobs;
                DROP TABLE IF EXISTS workers;
                DROP TABLE IF EXISTS queues;
                DROP TABLE IF EXISTS jobs;
                DELETE FROM schema_migrations WHERE version = 1;
                "
            )?;
        }
        _ => {
            return Err(crate::DbError::Migration(format!("Unknown version: {}", version)));
        }
    }

    Ok(())
}

/// Get current schema version
pub fn current_version(conn: &Connection) -> DbResult<i32> {
    let version = conn
        .query_row(
            "SELECT COALESCE(MAX(version), 0) FROM schema_migrations",
            [],
            |row| row.get(0),
        )?;

    Ok(version)
}

/// Check if migrations are needed
pub fn needs_migration(conn: &Connection) -> DbResult<bool> {
    let current = current_version(conn)?;
    Ok(current < CURRENT_VERSION)
}
