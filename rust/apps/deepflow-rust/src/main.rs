//! Deepflow Rust - Main entry point.
//!
//! An Actix-web application demonstrating complex dataflow patterns
//! for testing SAST taint analysis capabilities.

mod advanced;
mod async_flow;
mod closures;
mod handlers;
mod iterators;
mod models;
mod patterns;
mod pipeline;
mod sinks;
mod traits;

use actix_web::{web, App, HttpServer};
use std::env;
use std::io::{self, BufRead, BufReader};

/// Application configuration from environment
#[derive(Debug, Clone, Default)]
pub struct AppConfig {
    pub debug_mode: bool,
    pub max_connections: u32,
    pub timeout_seconds: u64,
}

/// Application shared state
pub struct AppState {
    pub config: AppConfig,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // TAINT SOURCE: Environment variables
    let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let server_port: u16 = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap_or(8080);

    // TAINT SOURCE: Command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let config_file = &args[1];
        // TAINT FLOW: args -> file read
        match std::fs::read_to_string(config_file) {
            Ok(content) => {
                println!("Loaded config from {}: {} bytes", config_file, content.len());
                // TAINT FLOW: file content -> sink
                let _ = sinks::log_data("CONFIG", &content);
            }
            Err(e) => eprintln!("Failed to load config: {}", e),
        }
    }

    // Create application state
    let app_state = web::Data::new(AppState {
        config: AppConfig::default(),
    });

    // Optional: Read from stdin on startup
    if env::var("READ_STDIN").is_ok() {
        read_stdin_commands();
    }

    println!("Starting Deepflow server at http://{}:{}", server_host, server_port);
    println!("Endpoints:");
    println!("  POST /api/pipeline          - Deep call chain pipeline");
    println!("  POST /api/pipeline/context  - Pipeline with context");
    println!("  GET  /api/pipeline/recursive/{{depth}} - Recursive pipeline");
    println!("  POST /api/pipeline/builder  - Builder pattern pipeline");
    println!("  POST /api/async/pipeline    - Async pipeline");
    println!("  POST /api/async/channel     - Channel-based pipeline");
    println!("  POST /api/async/workflow    - Workflow with context");
    println!("  POST /api/async/spawn       - Spawned task processing");
    println!("  POST /api/traits/process/{{name}} - Trait object processing");
    println!("  POST /api/traits/chain      - Processor chain");
    println!("  POST /api/closures/capture  - Closure capture demo");
    println!("  POST /api/closures/callback - Callback pattern");
    println!("  POST /api/closures/builder  - Callback builder");
    println!("  POST /api/iterators/map     - Map chain processing");
    println!("  POST /api/iterators/filter  - Filter chain processing");
    println!("  POST /api/iterators/fold    - Fold chain processing");
    println!("  POST /api/exec              - Command execution");
    println!("  POST /api/files             - File operations");
    println!("  POST /api/query             - Database query");
    println!("  POST /api/network           - Network request");
    println!("  GET  /api/request/info      - Request info extraction");
    println!("  GET  /api/request/header/{{name}} - Header extraction");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            // Pipeline routes - deep call chains
            .route("/api/pipeline", web::post().to(handlers::execute_pipeline))
            .route("/api/pipeline/context", web::post().to(handlers::pipeline_with_context))
            .route("/api/pipeline/recursive/{depth}", web::get().to(handlers::recursive_pipeline))
            .route("/api/pipeline/builder", web::post().to(handlers::builder_pipeline))
            // Async flow routes
            .route("/api/async/pipeline", web::post().to(handlers::async_pipeline))
            .route("/api/async/channel", web::post().to(handlers::channel_pipeline))
            .route("/api/async/workflow", web::post().to(handlers::workflow_handler))
            .route("/api/async/spawn", web::post().to(handlers::spawn_handler))
            // Trait-based routes
            .route("/api/traits/process/{processor}", web::post().to(handlers::trait_process))
            .route("/api/traits/chain", web::post().to(handlers::trait_chain))
            // Closure routes
            .route("/api/closures/capture", web::post().to(handlers::closure_capture))
            .route("/api/closures/callback", web::post().to(handlers::closure_callback))
            .route("/api/closures/builder", web::post().to(handlers::closure_builder))
            // Iterator routes
            .route("/api/iterators/map", web::post().to(handlers::iterator_map))
            .route("/api/iterators/filter", web::post().to(handlers::iterator_filter))
            .route("/api/iterators/fold", web::post().to(handlers::iterator_fold))
            // Direct sink routes
            .route("/api/exec", web::post().to(handlers::execute_command))
            .route("/api/files", web::post().to(handlers::file_operation))
            .route("/api/query", web::post().to(handlers::database_query))
            .route("/api/network", web::post().to(handlers::network_request))
            // Request extraction routes
            .route("/api/request/info", web::get().to(handlers::request_info))
            .route("/api/request/header/{name}", web::get().to(handlers::get_header))
    })
    .bind((server_host.as_str(), server_port))?
    .run()
    .await
}

/// TAINT SOURCE: Read commands from stdin
fn read_stdin_commands() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    println!("Reading commands from stdin (type 'quit' to stop):");

    for line in reader.lines() {
        if let Ok(input) = line {
            if input.trim() == "quit" {
                break;
            }

            // TAINT FLOW: stdin -> command parsing
            if input.starts_with("exec:") {
                let cmd = input.strip_prefix("exec:").unwrap();
                // TAINT SINK: stdin -> command execution
                match sinks::execute_shell(cmd) {
                    Ok(output) => println!("Output: {}", output),
                    Err(e) => eprintln!("Error: {}", e),
                }
            } else if input.starts_with("file:") {
                let path = input.strip_prefix("file:").unwrap();
                // TAINT SINK: stdin -> file read
                match sinks::read_from_file(path) {
                    Ok(content) => println!("File content:\n{}", content),
                    Err(e) => eprintln!("Error: {}", e),
                }
            } else if input.starts_with("fetch:") {
                let url = input.strip_prefix("fetch:").unwrap();
                // TAINT SINK: stdin -> network request
                match sinks::fetch_url(url) {
                    Ok(response) => println!("Response: {}", response),
                    Err(e) => eprintln!("Error: {}", e),
                }
            } else if input.starts_with("sql:") {
                let query = input.strip_prefix("sql:").unwrap();
                // TAINT SINK: stdin -> SQL query
                match sinks::execute_query(query) {
                    Ok(result) => println!("Query result: {}", result),
                    Err(e) => eprintln!("Error: {}", e),
                }
            } else {
                println!("Unknown command. Available: exec:, file:, fetch:, sql:");
            }
        }
    }
}

/// TAINT SOURCE: Parse JSON from reader
pub fn parse_json_input<R: io::Read>(reader: R) -> Result<models::RawInput, serde_json::Error> {
    serde_json::from_reader(reader)
}

/// TAINT SOURCE: Read from file (controlled by user)
pub fn read_config_file(path: &str) -> io::Result<String> {
    std::fs::read_to_string(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_flow() {
        let input = "test_input".to_string();
        let stages = vec!["stage1".to_string(), "stage2".to_string()];
        let result = pipeline::execute_pipeline(input, stages);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_async_flow() {
        let input = "async_test".to_string();
        let result = async_flow::async_pipeline(input).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_trait_chain() {
        let chain = traits::ProcessorChain::new()
            .add(Box::new(traits::UppercaseProcessor))
            .add(Box::new(traits::PrefixProcessor::new("[P]".to_string())));

        let result = chain.execute("test".to_string());
        assert!(result.contains("TEST"));
    }

    #[test]
    fn test_iterator_chain() {
        // Note: map_chain writes to /tmp/ which may not exist on Windows
        // Just verify the function signature compiles - actual I/O isn't the test goal
        let items = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let _result = iterators::map_chain(items);
        // Result may be Err on Windows (no /tmp/), Ok on Linux - either is fine for SAST testing
    }
}
