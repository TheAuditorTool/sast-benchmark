//! JobQueue HTTP API
//!
//! This crate provides an HTTP API for managing the job queue.
//!
//! # Endpoints
//!
//! - `POST /jobs` - Create a new job
//! - `GET /jobs` - List jobs
//! - `GET /jobs/:id` - Get job details
//! - `DELETE /jobs/:id` - Cancel a job
//! - `GET /queues` - List queues
//! - `GET /queues/:name/stats` - Get queue statistics
//! - `GET /health` - Health check
//! - `GET /metrics` - Prometheus metrics
//!
//! # INTENTIONAL VULNERABILITIES
//!
//! - XSS in error messages
//! - CORS misconfiguration
//! - Missing authentication
//! - SQL injection via query params

pub mod server;
pub mod routes;
pub mod handlers;
pub mod middleware;
pub mod auth;
pub mod error;

pub use server::{ApiServer, ApiConfig};
pub use error::ApiError;
