//! Calorie Counter App - A full-featured calorie tracking web application.
//!
//! This app demonstrates comprehensive taint analysis patterns:
//! - Multi-hop flows: HTTP handler -> service -> repository -> SQL
//! - Cross-file flows: Data flowing across multiple source files
//! - Validation transforms: Input sanitization before reaching sinks
//!
//! Architecture:
//! ```
//! [HTTP Request]
//!       │
//!       ▼
//! ┌─────────────┐
//! │  Handlers   │  ← TAINT SOURCE (web::Json, web::Path, web::Query)
//! └─────────────┘
//!       │
//!       ▼
//! ┌─────────────┐
//! │ Validation  │  ← TAINT TRANSFORM (validates/rejects)
//! └─────────────┘
//!       │
//!       ▼
//! ┌─────────────┐
//! │  Services   │  ← TAINT TRANSFORM (business logic)
//! └─────────────┘
//!       │
//!       ▼
//! ┌─────────────┐
//! │Repositories │  ← TAINT SINK (SQL queries)
//! └─────────────┘
//!       │
//!       ▼
//!  [Database]
//! ```

// Module declarations
pub mod config;
pub mod db;
pub mod errors;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod services;
pub mod validation;

// Re-exports for convenient access
pub use config::AppConfig;
pub use db::DbPool;
pub use errors::AppError;
