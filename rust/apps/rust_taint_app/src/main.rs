//! Main entry point for TheAuditor Rust taint analysis test app.
//!
//! This application demonstrates various taint patterns for security analysis:
//! - HTTP request input handling (sources)
//! - Command execution (sinks)
//! - SQL queries (sinks)
//! - File operations (sinks)
//! - Network requests (sinks)
//! - Unsafe operations (sinks)

mod commands;
mod database;
mod files;
mod handlers;
mod models;
mod network;
mod traits;
mod memory_ops;

use actix_web::{web, App, HttpServer};
use std::env;
use std::io::{self, BufRead, BufReader};

/// Application state shared across handlers
pub struct AppState {
    pub db_pool: sqlx::SqlitePool,
    pub config: models::AppConfig,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // TAINT SOURCE: Environment variables
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());
    let server_host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let server_port: u16 = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap_or(8080);

    // TAINT SOURCE: Command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let config_file = &args[1];
        // TAINT FLOW: args -> file read
        let config_content = std::fs::read_to_string(config_file)
            .expect("Failed to read config file");
        println!("Loaded config: {}", config_content);
    }

    // Create database pool
    let pool = sqlx::SqlitePool::connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Application state
    let app_state = web::Data::new(AppState {
        db_pool: pool,
        config: models::AppConfig::default(),
    });

    // TAINT SOURCE: stdin reading
    read_user_input();

    println!("Starting server at {}:{}", server_host, server_port);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            // User management routes
            .route("/users", web::get().to(handlers::get_users))
            .route("/users/{id}", web::get().to(handlers::get_user_by_id))
            .route("/users", web::post().to(handlers::create_user))
            .route("/users/search", web::get().to(handlers::search_users))
            // Command execution routes (dangerous!)
            .route("/admin/exec", web::post().to(handlers::execute_command))
            .route("/admin/shell", web::post().to(handlers::run_shell_command))
            // File operation routes
            .route("/files/read", web::get().to(handlers::read_file_handler))
            .route("/files/write", web::post().to(handlers::write_file_handler))
            // Network proxy routes (SSRF vectors)
            .route("/proxy", web::get().to(handlers::proxy_request))
            .route("/fetch", web::post().to(handlers::fetch_url))
            // Unsafe operations
            .route("/unsafe/transmute", web::post().to(handlers::unsafe_transmute_handler))
    })
    .bind((server_host.as_str(), server_port))?
    .run()
    .await
}

/// TAINT SOURCE: Read input from stdin
fn read_user_input() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    println!("Enter commands (type 'quit' to exit):");
    for line in reader.lines() {
        if let Ok(input) = line {
            if input.trim() == "quit" {
                break;
            }
            // TAINT FLOW: stdin -> command execution
            if input.starts_with("exec:") {
                let cmd = input.strip_prefix("exec:").unwrap();
                commands::execute_shell_command(cmd);
            }
        }
    }
}

/// TAINT SOURCE: Read from file (controlled by user)
pub fn read_file(path: &str) -> io::Result<String> {
    std::fs::read_to_string(path)
}

/// TAINT SOURCE: Parse JSON from reader
pub fn parse_json_input<R: io::Read>(reader: R) -> Result<models::UserInput, serde_json::Error> {
    serde_json::from_reader(reader)
}
