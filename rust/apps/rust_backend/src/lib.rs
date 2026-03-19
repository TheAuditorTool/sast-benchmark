//! Rust Backend Library
//!
//! Re-exports modules for the vulnerable Rust backend.

pub mod handlers;
pub mod unsafe_ops;
pub mod vulnerable;

// Extern crate declarations for lazy_static
#[macro_use]
extern crate lazy_static;
