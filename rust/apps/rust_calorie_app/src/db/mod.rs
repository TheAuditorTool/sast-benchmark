//! Database module - pool management, migrations, and schema.
//!
//! Multi-hop taint flow: config (SOURCE) -> pool creation -> query execution (SINK)

pub mod migrations;
pub mod pool;

pub use migrations::run_migrations;
pub use pool::{create_pool, DbPool};
