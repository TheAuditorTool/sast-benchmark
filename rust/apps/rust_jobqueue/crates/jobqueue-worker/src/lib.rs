//! JobQueue Worker
//!
//! This crate provides the worker implementation for processing jobs.
//!
//! # Features
//!
//! - Concurrent job processing
//! - Graceful shutdown
//! - Job timeouts
//! - Retry handling
//! - Event emission
//! - Health checks
//!
//! # Example
//!
//! ```rust,ignore
//! use jobqueue_worker::{Worker, WorkerConfig};
//! use jobqueue_db::SqliteStore;
//!
//! #[tokio::main]
//! async fn main() {
//!     let store = SqliteStore::new("jobs.db").unwrap();
//!     let config = WorkerConfig::default();
//!
//!     let mut worker = Worker::new(store, config);
//!     worker.register("email", send_email);
//!     worker.start().await;
//! }
//! ```

pub mod worker;
pub mod executor;
pub mod scheduler;
pub mod health;
pub mod handlers;
pub mod middleware;

pub use worker::{Worker, WorkerConfig, WorkerHandle};
pub use executor::{ExecutorPool, TaskHandle};
pub use scheduler::Scheduler;
pub use health::{HealthChecker, HealthStatus};
pub use middleware::{Middleware, MiddlewareStack};
