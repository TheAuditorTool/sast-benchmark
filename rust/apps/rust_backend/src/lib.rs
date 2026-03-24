//! Rust Backend Library
//!
//! Re-exports modules for the vulnerable Rust backend.

pub mod handlers;
pub mod memory_ops;
pub mod patterns;

// Extern crate declarations for lazy_static
#[macro_use]
extern crate lazy_static;
