//! Shared types for standalone benchmark test cases.
//!
//! Each test file is a pure function: BenchmarkRequest -> BenchmarkResponse.
//! No framework dependency. Thin adapters convert HTTP to BenchmarkRequest.
//!
//! Pattern matches Go benchmark (stdlib handler) and Java benchmark (Servlet class).

use std::collections::HashMap;

/// Abstraction over HTTP request — framework-agnostic.
/// Adapters for actix-web, axum, rocket, warp convert their native
/// request types into this struct before calling the test function.
pub struct BenchmarkRequest {
    pub query_params: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl BenchmarkRequest {
    pub fn param(&self, name: &str) -> String {
        self.query_params.get(name).cloned().unwrap_or_default()
    }

    pub fn cookie(&self, name: &str) -> String {
        self.cookies.get(name).cloned().unwrap_or_default()
    }

    pub fn header(&self, name: &str) -> String {
        self.headers.get(name).cloned().unwrap_or_default()
    }

    pub fn body_str(&self) -> String {
        self.body.clone().unwrap_or_default()
    }
}

/// Abstraction over HTTP response.
pub struct BenchmarkResponse {
    pub status: u16,
    pub body: String,
}

impl BenchmarkResponse {
    pub fn ok(body: &str) -> Self {
        Self { status: 200, body: body.to_string() }
    }

    pub fn error(msg: &str) -> Self {
        Self { status: 500, body: msg.to_string() }
    }

    pub fn bad_request(msg: &str) -> Self {
        Self { status: 400, body: msg.to_string() }
    }

    pub fn forbidden(msg: &str) -> Self {
        Self { status: 403, body: msg.to_string() }
    }
}
