//! JobQueue Database Layer
//!
//! This crate provides database implementations for storing jobs.
//!
//! # Supported Databases
//!
//! - SQLite (default, embedded)
//! - PostgreSQL (planned)
//! - MySQL (planned)
//!
//! # INTENTIONAL VULNERABILITIES FOR TESTING
//!
//! This crate contains intentional security vulnerabilities:
//! - SQL Injection in search functions
//! - Path traversal in backup functions
//! - Weak transaction isolation
//! - Race conditions in concurrent operations

pub mod sqlite;
pub mod migrations;
pub mod pool;
pub mod queries;
pub mod backup;

pub use sqlite::SqliteStore;
pub use pool::{ConnectionPool, PoolConfig};

/// Database result type
pub type DbResult<T> = Result<T, DbError>;

/// Database error type
#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Query error: {0}")]
    Query(String),

    #[error("Migration error: {0}")]
    Migration(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Constraint violation: {0}")]
    Constraint(String),

    #[error("Pool exhausted")]
    PoolExhausted,

    #[error("Transaction error: {0}")]
    Transaction(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("SQLite error: {0}")]
    Sqlite(String),
}

impl From<rusqlite::Error> for DbError {
    fn from(e: rusqlite::Error) -> Self {
        DbError::Sqlite(e.to_string())
    }
}

impl From<DbError> for jobqueue_core::JobQueueError {
    fn from(e: DbError) -> Self {
        jobqueue_core::JobQueueError::DatabaseError {
            message: e.to_string(),
            query: None,
        }
    }
}
