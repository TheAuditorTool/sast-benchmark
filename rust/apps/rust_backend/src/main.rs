//! Rust Backend - Intentionally Vulnerable for SAST Testing
//!
//! This backend demonstrates various Rust-specific vulnerabilities
//! for cross-boundary taint analysis testing.
//!
//! VULNERABILITIES:
//! 1. SQL Injection via raw SQL
//! 2. Command Injection via std::process::Command
//! 3. Path Traversal via std::fs
//! 4. Unsafe memory operations
//! 5. Integer overflow
//! 6. Use-after-free patterns
//! 7. Buffer overflow in unsafe blocks
//! 8. Race conditions

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;

mod handlers;
mod unsafe_ops;
mod vulnerable;

// -----------------------------------------------------------------------------
// Request/Response Types
// -----------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub sort: Option<String>,
    pub limit: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ExecRequest {
    pub command: String,
    pub args: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct FileRequest {
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct BufferRequest {
    pub size: usize,
    pub data: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FormatRequest {
    pub template: String,
    pub values: std::collections::HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct DeserializeRequest {
    pub payload: String,
    pub format: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

// -----------------------------------------------------------------------------
// VULNERABILITY #1: SQL Injection via raw SQL
// Source: query.q (user input)
// Sink: raw SQL query execution
// -----------------------------------------------------------------------------
async fn search_users(query: web::Query<SearchQuery>) -> impl Responder {
    let search_term = &query.q;
    let sort_by = query.sort.as_deref().unwrap_or("username");
    let limit = query.limit.unwrap_or(100);

    // TAINT SINK: User input directly interpolated into SQL
    // Attacker payload: q=' OR '1'='1' --
    let sql = format!(
        "SELECT id, username, email FROM users WHERE username LIKE '%{}%' ORDER BY {} LIMIT {}",
        search_term, sort_by, limit
    );

    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "query": sql,
            "message": "Query would be executed",
            "injection_point": search_term
        })),
        error: None,
    })
}

// -----------------------------------------------------------------------------
// VULNERABILITY #2: Command Injection
// Source: req.command (user input)
// Sink: std::process::Command
// -----------------------------------------------------------------------------
async fn execute_command(req: web::Json<ExecRequest>) -> impl Responder {
    // TAINT SINK: User controlled command execution
    // Attacker payload: command="sh", args=["-c", "cat /etc/passwd"]
    let output = Command::new(&req.command)
        .args(&req.args)
        .output();

    match output {
        Ok(out) => HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(serde_json::json!({
                "stdout": String::from_utf8_lossy(&out.stdout),
                "stderr": String::from_utf8_lossy(&out.stderr),
                "exit_code": out.status.code(),
                "command": &req.command,
                "args": &req.args
            })),
            error: None,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

// -----------------------------------------------------------------------------
// VULNERABILITY #3: Path Traversal
// Source: query.path (user input)
// Sink: std::fs::read_to_string
// -----------------------------------------------------------------------------
async fn read_file(query: web::Query<FileRequest>) -> impl Responder {
    // TAINT SINK: User controlled file path without sanitization
    // Attacker payload: path=../../../etc/passwd
    let file_path = &query.path;

    match fs::read_to_string(file_path) {
        Ok(content) => HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(serde_json::json!({
                "content": content,
                "path": file_path,
                "size": content.len()
            })),
            error: None,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            success: false,
            data: None,
            error: Some(format!("Failed to read {}: {}", file_path, e)),
        }),
    }
}

// -----------------------------------------------------------------------------
// VULNERABILITY #4: Unsafe Buffer Operations
// Source: req.size (user controlled)
// Sink: unsafe { Vec::set_len() }
// -----------------------------------------------------------------------------
async fn allocate_buffer(req: web::Json<BufferRequest>) -> impl Responder {
    // TAINT SINK: User controlled size in unsafe block
    // This can cause buffer overflow or memory corruption
    let result = unsafe_ops::dangerous_buffer_alloc(req.size);

    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "allocated_size": req.size,
            "result": result
        })),
        error: None,
    })
}

// -----------------------------------------------------------------------------
// VULNERABILITY #5: Format String (simulated)
// Source: req.template (user controlled)
// Sink: format! macro (via runtime string building)
// -----------------------------------------------------------------------------
async fn format_template(req: web::Json<FormatRequest>) -> impl Responder {
    // TAINT SINK: User controlled template
    // While Rust's format! is compile-time safe, we simulate runtime formatting
    let mut result = req.template.clone();
    for (key, value) in &req.values {
        let placeholder = format!("{{{}}}", key);
        result = result.replace(&placeholder, value);
    }

    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "rendered": result,
            "template": &req.template
        })),
        error: None,
    })
}

// -----------------------------------------------------------------------------
// VULNERABILITY #6: Unsafe Deserialization
// Source: req.payload (user controlled)
// Sink: bincode::deserialize
// -----------------------------------------------------------------------------
async fn deserialize_payload(req: web::Json<DeserializeRequest>) -> impl Responder {
    let result = unsafe_ops::dangerous_deserialize(&req.payload);

    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "result": result,
            "payload_len": req.payload.len()
        })),
        error: None,
    })
}

// -----------------------------------------------------------------------------
// VULNERABILITY #7: SSRF (Server-Side Request Forgery)
// Source: url query parameter
// Sink: reqwest::get()
// -----------------------------------------------------------------------------
async fn fetch_url(query: web::Query<std::collections::HashMap<String, String>>) -> impl Responder {
    let url = match query.get("url") {
        Some(u) => u,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            success: false,
            data: None,
            error: Some("Missing 'url' parameter".to_string()),
        }),
    };

    // TAINT SINK: User controlled URL for server-side request
    // Attacker payload: url=http://169.254.169.254/latest/meta-data/
    match reqwest::get(url).await {
        Ok(response) => {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();

            HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(serde_json::json!({
                    "status": status,
                    "body": body.chars().take(1000).collect::<String>(),
                    "url": url
                })),
                error: None,
            })
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

// -----------------------------------------------------------------------------
// VULNERABILITY #8: Integer Overflow
// Source: query parameters
// Sink: arithmetic operations
// -----------------------------------------------------------------------------
async fn calculate(query: web::Query<std::collections::HashMap<String, String>>) -> impl Responder {
    let a: i32 = query.get("a").and_then(|s| s.parse().ok()).unwrap_or(0);
    let b: i32 = query.get("b").and_then(|s| s.parse().ok()).unwrap_or(0);
    let op = query.get("op").map(|s| s.as_str()).unwrap_or("add");

    // TAINT SINK: Integer overflow without checked arithmetic
    // Attacker payload: a=2147483647&b=1&op=add
    let result = match op {
        "add" => a + b,           // Can overflow!
        "mul" => a * b,           // Can overflow!
        "sub" => a - b,           // Can underflow!
        "shl" => a << (b as u32), // Can overflow!
        _ => 0,
    };

    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "a": a,
            "b": b,
            "op": op,
            "result": result
        })),
        error: None,
    })
}

// -----------------------------------------------------------------------------
// Main server setup
// -----------------------------------------------------------------------------
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv::dotenv().ok();

    let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string());

    println!("🦀 Rust backend starting on {}", bind_addr);
    println!("⚠️  WARNING: This server contains intentional vulnerabilities!");

    HttpServer::new(|| {
        // VULNERABILITY: Permissive CORS
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .route("/api/users/search", web::get().to(search_users))
            .route("/api/exec", web::post().to(execute_command))
            .route("/api/files/read", web::get().to(read_file))
            .route("/api/buffer/allocate", web::post().to(allocate_buffer))
            .route("/api/format", web::post().to(format_template))
            .route("/api/deserialize", web::post().to(deserialize_payload))
            .route("/api/fetch", web::get().to(fetch_url))
            .route("/api/calc", web::get().to(calculate))
            // Additional vulnerable routes from handlers module
            .configure(handlers::configure_routes)
            .configure(vulnerable::configure_routes)
    })
    .bind(&bind_addr)?
    .run()
    .await
}
