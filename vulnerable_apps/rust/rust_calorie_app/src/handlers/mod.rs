//! HTTP handlers - Entry points for all HTTP requests.
//!
//! TAINT SOURCE: All handlers receive data from HTTP requests via:
//! - web::Json (request body)
//! - web::Path (URL path parameters)
//! - web::Query (URL query parameters)
//! - web::Form (form data)
//!
//! Multi-hop flow: handlers (SOURCE) -> services (TRANSFORM) -> repositories (SINK)

pub mod auth;
pub mod meals;
pub mod reports;
pub mod schedules;
pub mod workouts;

pub use auth::*;
pub use meals::*;
pub use reports::*;
pub use schedules::*;
pub use workouts::*;
