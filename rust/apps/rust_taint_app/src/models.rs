//! Data models for the taint test application.
//!
//! These models demonstrate derive macros and struct definitions
//! that TheAuditor's Rust extractor should capture.

use serde::{Deserialize, Serialize};

/// User input from HTTP requests - TAINT SOURCE
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInput {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: Option<String>,
}

/// Query parameters for user search - TAINT SOURCE
#[derive(Debug, Clone, Deserialize)]
pub struct UserSearchQuery {
    pub name: Option<String>,
    pub email: Option<String>,
    pub sort_by: Option<String>,
    pub order: Option<String>,
    pub limit: Option<i32>,
}

/// Command execution request - TAINT SOURCE (dangerous!)
#[derive(Debug, Clone, Deserialize)]
pub struct CommandRequest {
    pub command: String,
    pub args: Vec<String>,
    pub working_dir: Option<String>,
}

/// Shell command request - TAINT SOURCE (very dangerous!)
#[derive(Debug, Clone, Deserialize)]
pub struct ShellCommandRequest {
    pub shell_command: String,
    pub env_vars: Option<std::collections::HashMap<String, String>>,
}

/// File operation request - TAINT SOURCE
#[derive(Debug, Clone, Deserialize)]
pub struct FileRequest {
    pub path: String,
    pub content: Option<String>,
}

/// Network proxy request - TAINT SOURCE (SSRF vector)
#[derive(Debug, Clone, Deserialize)]
pub struct ProxyRequest {
    pub url: String,
    pub method: Option<String>,
    pub headers: Option<std::collections::HashMap<String, String>>,
    pub body: Option<String>,
}

/// User entity for database operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: String,
}

/// Database row representation
#[derive(Debug, Clone)]
pub struct UserRow {
    pub id: i64,
    pub username: String,
    pub email: String,
}

/// Application configuration
#[derive(Debug, Clone, Default)]
pub struct AppConfig {
    pub debug_mode: bool,
    pub max_connections: u32,
    pub timeout_seconds: u64,
}

/// Unsafe data container - for demonstrating unsafe operations
#[repr(C)]
pub struct UnsafeDataContainer {
    pub ptr: *mut u8,
    pub len: usize,
    pub capacity: usize,
}

impl UnsafeDataContainer {
    /// Create a new unsafe container
    pub fn new(data: Vec<u8>) -> Self {
        let mut data = std::mem::ManuallyDrop::new(data);
        Self {
            ptr: data.as_mut_ptr(),
            len: data.len(),
            capacity: data.capacity(),
        }
    }

    /// UNSAFE: Read data from raw pointer
    pub unsafe fn read_data(&self) -> Vec<u8> {
        // SAFETY: ptr, len, capacity come from a valid Vec
        std::slice::from_raw_parts(self.ptr, self.len).to_vec()
    }
}

/// Generic wrapper for taint tracking demonstration
#[derive(Debug, Clone)]
pub struct Tainted<T> {
    pub value: T,
    pub source: String,
}

impl<T> Tainted<T> {
    pub fn new(value: T, source: &str) -> Self {
        Self {
            value,
            source: source.to_string(),
        }
    }

    pub fn unwrap(self) -> T {
        self.value
    }
}

/// Response types
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message.to_string()),
        }
    }
}
