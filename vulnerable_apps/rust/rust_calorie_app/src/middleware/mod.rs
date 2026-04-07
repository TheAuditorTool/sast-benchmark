//! Middleware module - Authentication and request processing.
//!
//! TAINT SOURCE: JWT tokens from Authorization headers are taint sources.
//! The middleware extracts and validates tokens, passing user_id downstream.

mod auth;

pub use auth::AuthMiddleware;
