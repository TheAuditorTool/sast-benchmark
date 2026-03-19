//! Error types for JobQueue
//!
//! This module defines all error types used throughout the library.

use std::fmt;
use thiserror::Error;

/// Result type alias using JobQueueError
pub type Result<T> = std::result::Result<T, JobQueueError>;

/// Main error enum for all JobQueue operations
#[derive(Error, Debug)]
pub enum JobQueueError {
    /// Job was not found in the queue
    #[error("Job not found: {job_id}")]
    JobNotFound { job_id: String },

    /// Queue does not exist
    #[error("Queue not found: {queue_name}")]
    QueueNotFound { queue_name: String },

    /// Worker registration failed
    #[error("Worker registration failed: {reason}")]
    WorkerRegistrationFailed { reason: String },

    /// Job execution failed
    #[error("Job execution failed: {message}")]
    ExecutionFailed {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// Database error
    #[error("Database error: {message}")]
    DatabaseError {
        message: String,
        query: Option<String>,
    },

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Timeout occurred
    #[error("Operation timed out after {duration_ms}ms")]
    Timeout { duration_ms: u64 },

    /// Queue is full
    #[error("Queue {queue_name} is full (max: {max_size})")]
    QueueFull { queue_name: String, max_size: usize },

    /// Invalid state transition
    #[error("Invalid state transition from {from:?} to {to:?}")]
    InvalidStateTransition {
        from: crate::job::JobState,
        to: crate::job::JobState,
    },

    /// Permission denied
    #[error("Permission denied: {action}")]
    PermissionDenied { action: String },

    /// Rate limit exceeded
    #[error("Rate limit exceeded: {limit}/s")]
    RateLimitExceeded { limit: u32 },

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),

    /// IO Error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

impl JobQueueError {
    /// Create a new execution error with source
    pub fn execution<E>(message: impl Into<String>, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::ExecutionFailed {
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Create a database error with query context
    pub fn database(message: impl Into<String>, query: Option<String>) -> Self {
        Self::DatabaseError {
            message: message.into(),
            query,
        }
    }

    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::Timeout { .. }
                | Self::DatabaseError { .. }
                | Self::RateLimitExceeded { .. }
        )
    }

    /// Get error code for API responses
    pub fn error_code(&self) -> &'static str {
        match self {
            Self::JobNotFound { .. } => "JOB_NOT_FOUND",
            Self::QueueNotFound { .. } => "QUEUE_NOT_FOUND",
            Self::WorkerRegistrationFailed { .. } => "WORKER_REGISTRATION_FAILED",
            Self::ExecutionFailed { .. } => "EXECUTION_FAILED",
            Self::SerializationError(_) => "SERIALIZATION_ERROR",
            Self::DatabaseError { .. } => "DATABASE_ERROR",
            Self::ConfigError(_) => "CONFIG_ERROR",
            Self::Timeout { .. } => "TIMEOUT",
            Self::QueueFull { .. } => "QUEUE_FULL",
            Self::InvalidStateTransition { .. } => "INVALID_STATE",
            Self::PermissionDenied { .. } => "PERMISSION_DENIED",
            Self::RateLimitExceeded { .. } => "RATE_LIMIT",
            Self::Internal(_) => "INTERNAL_ERROR",
            Self::IoError(_) => "IO_ERROR",
        }
    }
}

/// Error context wrapper for adding additional context to errors
#[derive(Debug)]
pub struct ErrorContext<E> {
    error: E,
    context: String,
    location: Option<ErrorLocation>,
}

#[derive(Debug, Clone)]
pub struct ErrorLocation {
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

impl<E: std::error::Error> fmt::Display for ErrorContext<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.context, self.error)?;
        if let Some(ref loc) = self.location {
            write!(f, " at {}:{}:{}", loc.file, loc.line, loc.column)?;
        }
        Ok(())
    }
}

impl<E: std::error::Error + 'static> std::error::Error for ErrorContext<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

/// Extension trait for adding context to errors
pub trait ErrorExt<T, E> {
    fn context(self, ctx: impl Into<String>) -> std::result::Result<T, ErrorContext<E>>;
    fn with_location(self, file: &'static str, line: u32, column: u32)
        -> std::result::Result<T, ErrorContext<E>>;
}

impl<T, E: std::error::Error> ErrorExt<T, E> for std::result::Result<T, E> {
    fn context(self, ctx: impl Into<String>) -> std::result::Result<T, ErrorContext<E>> {
        self.map_err(|e| ErrorContext {
            error: e,
            context: ctx.into(),
            location: None,
        })
    }

    fn with_location(
        self,
        file: &'static str,
        line: u32,
        column: u32
    ) -> std::result::Result<T, ErrorContext<E>> {
        self.map_err(|e| ErrorContext {
            error: e,
            context: String::new(),
            location: Some(ErrorLocation { file, line, column }),
        })
    }
}

/// Macro for creating errors with location info
#[macro_export]
macro_rules! err {
    ($err:expr) => {
        Err($err).with_location(file!(), line!(), column!())
    };
    ($err:expr, $ctx:expr) => {
        Err($err).context($ctx).map_err(|e| ErrorContext {
            location: Some(ErrorLocation {
                file: file!(),
                line: line!(),
                column: column!(),
            }),
            ..e
        })
    };
}
