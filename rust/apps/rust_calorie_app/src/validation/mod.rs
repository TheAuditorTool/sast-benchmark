//! Validation module - Joi/Zod equivalent for Rust.
//!
//! TAINT TRANSFORM: All validators receive tainted input and either
//! return sanitized data or reject with validation errors.

mod validators;

pub use validators::Validators;
