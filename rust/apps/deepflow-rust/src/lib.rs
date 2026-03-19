//! Deepflow Rust - Complex dataflow patterns for taint analysis.
//!
//! This application demonstrates challenging dataflow patterns for SAST tools:
//!
//! - **Deep call chains**: Data flows through 5+ levels of function calls
//! - **Async/await chains**: Data flows through async functions and channels
//! - **Trait-based polymorphism**: Data flows through trait objects
//! - **Closure captures**: Data captured and transformed by closures
//! - **Iterator chains**: Data flows through iterator combinators
//!
//! ## Architecture
//!
//! ```text
//! HTTP Request (TAINT SOURCE)
//!        │
//!        ▼
//!    ┌─────────┐
//!    │ Handlers│ ◄── Actix-web extractors (Json, Path, Query)
//!    └────┬────┘
//!         │
//!    ┌────┴────────────────────────────────────┐
//!    │                                         │
//!    ▼                                         ▼
//! ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐
//! │ Pipeline │  │AsyncFlow │  │  Traits  │  │ Closures │
//! │ (deep    │  │(channels,│  │(dyn Trait│  │(captures,│
//! │  chains) │  │ spawns)  │  │ generics)│  │callbacks)│
//! └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘
//!      │             │             │             │
//!      └─────────────┴─────────────┴─────────────┘
//!                        │
//!                        ▼
//!                  ┌───────────┐
//!                  │   Sinks   │ ◄── Command exec, file I/O, SQL, network
//!                  └───────────┘
//! ```
//!
//! ## Modules
//!
//! - [`models`]: Data structures for requests and processing
//! - [`handlers`]: HTTP handlers (taint sources)
//! - [`pipeline`]: Deep synchronous call chains
//! - [`async_flow`]: Async/await and channel-based dataflow
//! - [`traits`]: Trait object and generic dataflow
//! - [`closures`]: Closure capture patterns
//! - [`iterators`]: Iterator chain dataflow
//! - [`sinks`]: Dangerous operations (command exec, file I/O, SQL, network)
//! - [`advanced`]: Macros, FFI, lifetimes, const generics, unsafe traits
//! - [`patterns`]: Match expressions, destructuring, tuple/unit structs

pub mod advanced;
pub mod async_flow;
pub mod closures;
pub mod handlers;
pub mod iterators;
pub mod models;
pub mod patterns;
pub mod pipeline;
pub mod sinks;
pub mod traits;

/// Re-export commonly used types
pub use models::{
    ApiResponse, AsyncWorkflowRequest, BatchRequest, CommandRequest,
    FileRequest, NetworkRequest, PipelineRequest, QueryRequest, RawInput,
};
