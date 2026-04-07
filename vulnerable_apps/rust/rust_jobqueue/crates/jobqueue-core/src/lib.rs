//! JobQueue Core Library
//!
//! This crate provides the core types, traits, and error definitions
//! for the JobQueue distributed task processing system.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
//! │   Client    │────>│    Queue    │────>│   Worker    │
//! └─────────────┘     └─────────────┘     └─────────────┘
//!       │                   │                   │
//!       └───────────────────┴───────────────────┘
//!                           │
//!                    ┌──────┴──────┐
//!                    │  Database   │
//!                    └─────────────┘
//! ```

pub mod error;
pub mod job;
pub mod priority;
pub mod result;
pub mod traits;
pub mod types;
pub mod config;
pub mod events;
pub mod metrics;

// Re-exports for convenience
pub use error::{JobQueueError, Result};
pub use job::{Job, JobBuilder, JobId, JobPayload, JobState};
pub use priority::Priority;
pub use traits::{JobExecutor, JobStore, QueueBackend};
pub use types::{WorkerId, QueueName, Timestamp};
pub use config::QueueConfig;
pub use events::{JobEvent, EventHandler};
pub use metrics::Metrics;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default queue name
pub const DEFAULT_QUEUE: &str = "default";

/// Maximum retry attempts
pub const MAX_RETRIES: u32 = 5;
