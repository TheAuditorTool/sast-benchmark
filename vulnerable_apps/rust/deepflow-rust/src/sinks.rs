//! Dangerous sink operations - these are the endpoints for tainted data.
//!
//! All functions in this module represent security-sensitive operations
//! that should be flagged when receiving tainted (user-controlled) data.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::process::{Command, Stdio};

// ============================================================================
// COMMAND EXECUTION SINKS
// ============================================================================

/// TAINT SINK: Execute shell command
/// CRITICAL: Direct shell execution with user data
pub fn execute_shell(command: &str) -> Result<String, String> {
    // TAINT SINK: std::process::Command with user-controlled shell command
    #[cfg(target_os = "windows")]
    let output = Command::new("cmd")
        .args(["/C", command])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| e.to_string())?;

    #[cfg(not(target_os = "windows"))]
    let output = Command::new("sh")
        .args(["-c", command])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| e.to_string())?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.to_string())
}

/// TAINT SINK: Execute program with arguments
pub fn execute_program(program: &str, args: &[String]) -> Result<String, String> {
    // TAINT SINK: Command::new with user-controlled program name
    let output = Command::new(program)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| e.to_string())?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// TAINT SINK: Execute with working directory
pub fn execute_in_dir(program: &str, working_dir: &str) -> Result<String, String> {
    // TAINT SINK: Both program and directory are user-controlled
    let output = Command::new(program)
        .current_dir(working_dir)
        .output()
        .map_err(|e| e.to_string())?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// TAINT SINK: Execute with environment variables
pub fn execute_with_env(
    program: &str,
    env_vars: &std::collections::HashMap<String, String>,
) -> Result<String, String> {
    // TAINT SINK: Command with user-controlled environment
    let mut cmd = Command::new(program);
    for (key, value) in env_vars {
        cmd.env(key, value);
    }
    let output = cmd.output().map_err(|e| e.to_string())?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

// ============================================================================
// FILE OPERATION SINKS
// ============================================================================

/// TAINT SINK: Write to file at user-controlled path
pub fn write_to_file(path: &str, content: &str) -> Result<String, String> {
    // TAINT SINK: std::fs::write with user-controlled path
    fs::write(path, content).map_err(|e| e.to_string())?;
    Ok(format!("Written to {}", path))
}

/// TAINT SINK: Read from user-controlled path
pub fn read_from_file(path: &str) -> Result<String, String> {
    // TAINT SINK: std::fs::read_to_string with user-controlled path
    fs::read_to_string(path).map_err(|e| e.to_string())
}

/// TAINT SINK: Append to file
pub fn append_to_file(path: &str, content: &str) -> Result<String, String> {
    // TAINT SINK: OpenOptions::open with user-controlled path
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .map_err(|e| e.to_string())?;

    file.write_all(content.as_bytes())
        .map_err(|e| e.to_string())?;

    Ok(format!("Appended to {}", path))
}

/// TAINT SINK: Create file at path
pub fn create_file(path: &str) -> Result<File, String> {
    // TAINT SINK: File::create with user-controlled path
    File::create(path).map_err(|e| e.to_string())
}

/// TAINT SINK: Delete file
pub fn delete_file(path: &str) -> Result<String, String> {
    // TAINT SINK: fs::remove_file with user-controlled path
    fs::remove_file(path).map_err(|e| e.to_string())?;
    Ok(format!("Deleted {}", path))
}

/// TAINT SINK: Create directory
pub fn create_directory(path: &str) -> Result<String, String> {
    // TAINT SINK: fs::create_dir_all with user-controlled path
    fs::create_dir_all(path).map_err(|e| e.to_string())?;
    Ok(format!("Created directory {}", path))
}

/// TAINT SINK: Copy file
pub fn copy_file(source: &str, dest: &str) -> Result<u64, String> {
    // TAINT SINK: fs::copy with user-controlled paths
    fs::copy(source, dest).map_err(|e| e.to_string())
}

/// TAINT SINK: Move/rename file
pub fn move_file(source: &str, dest: &str) -> Result<String, String> {
    // TAINT SINK: fs::rename with user-controlled paths
    fs::rename(source, dest).map_err(|e| e.to_string())?;
    Ok(format!("Moved {} to {}", source, dest))
}

// ============================================================================
// SQL/DATABASE SINKS
// ============================================================================

/// TAINT SINK: Execute raw SQL query
/// This simulates database execution - in real app would use sqlx/diesel
pub fn execute_query(query: &str) -> Result<String, String> {
    // TAINT SINK: Raw SQL execution
    // In real code: sqlx::query(query).execute(pool)
    println!("[DB] Executing: {}", query);

    // Simulate SQL execution
    if query.to_lowercase().contains("drop")
        || query.to_lowercase().contains("delete")
        || query.to_lowercase().contains("truncate")
    {
        // Dangerous operations detected
        return Err("Dangerous SQL operation blocked".to_string());
    }

    Ok(format!("Query executed: {}", query))
}

/// TAINT SINK: Execute parameterized query (still dangerous if query string is tainted)
pub fn execute_parameterized(query: &str, params: &[&str]) -> Result<String, String> {
    // TAINT SINK: If query itself is user-controlled, parameters don't help
    println!("[DB] Executing: {} with params: {:?}", query, params);
    Ok(format!("Parameterized query executed"))
}

/// TAINT SINK: Build and execute dynamic query
pub fn build_query(table: &str, columns: &[String], filter: Option<&str>) -> Result<String, String> {
    // TAINT SINK: Dynamic SQL construction
    let cols = if columns.is_empty() {
        "*".to_string()
    } else {
        columns.join(", ")
    };

    let mut query = format!("SELECT {} FROM {}", cols, table);

    if let Some(f) = filter {
        // TAINT SINK: Filter directly in SQL
        query.push_str(&format!(" WHERE {}", f));
    }

    execute_query(&query)
}

// ============================================================================
// NETWORK SINKS (SSRF)
// ============================================================================

/// TAINT SINK: Fetch URL (SSRF)
pub fn fetch_url(url: &str) -> Result<String, String> {
    // TAINT SINK: reqwest::get with user-controlled URL
    // In real async code: reqwest::get(url).await
    println!("[NET] Fetching: {}", url);

    // Simulate network request
    Ok(format!("Response from {}", url))
}

/// TAINT SINK: POST to URL
pub fn post_to_url(url: &str, body: &str) -> Result<String, String> {
    // TAINT SINK: HTTP POST with user-controlled URL and body
    println!("[NET] POSTing to {} with body: {}", url, body);
    Ok(format!("Posted to {}", url))
}

/// TAINT SINK: Connect to TCP address
pub fn connect_tcp(host: &str, port: u16) -> Result<String, String> {
    // TAINT SINK: TcpStream::connect with user-controlled address
    let addr = format!("{}:{}", host, port);
    println!("[NET] Connecting to {}", addr);

    // In real code: TcpStream::connect(&addr)
    Ok(format!("Connected to {}", addr))
}

/// TAINT SINK: DNS lookup
pub fn resolve_host(hostname: &str) -> Result<Vec<String>, String> {
    // TAINT SINK: DNS resolution with user-controlled hostname
    println!("[DNS] Resolving: {}", hostname);

    // Simulated resolution
    Ok(vec![format!("192.168.1.1 (resolved from {})", hostname)])
}

// ============================================================================
// UNSAFE MEMORY SINKS
// ============================================================================

/// TAINT SINK: Write to arbitrary address
pub unsafe fn write_to_address(address: usize, data: &[u8]) {
    // TAINT SINK: Raw pointer write with user-controlled address
    let ptr = address as *mut u8;
    std::ptr::copy_nonoverlapping(data.as_ptr(), ptr, data.len());
}

/// TAINT SINK: Read from arbitrary address
pub unsafe fn read_from_address(address: usize, len: usize) -> Vec<u8> {
    // TAINT SINK: Raw pointer read with user-controlled address
    let ptr = address as *const u8;
    let slice = std::slice::from_raw_parts(ptr, len);
    slice.to_vec()
}

/// TAINT SINK: Transmute user data
pub unsafe fn transmute_data<T, U>(data: T) -> U {
    // TAINT SINK: std::mem::transmute with user data
    std::mem::transmute_copy(&data)
}

// ============================================================================
// LOGGING/OUTPUT SINKS (can leak sensitive data)
// ============================================================================

/// TAINT SINK: Log user data (information disclosure)
pub fn log_data(level: &str, message: &str) {
    // TAINT SINK: Logging user-controlled data can leak sensitive info
    println!("[{}] {}", level, message);
}

/// TAINT SINK: Write to stderr
pub fn write_stderr(message: &str) -> io::Result<()> {
    // TAINT SINK: Writing user data to stderr
    eprintln!("{}", message);
    Ok(())
}

// ============================================================================
// SERIALIZATION SINKS
// ============================================================================

/// TAINT SINK: Deserialize user data (can trigger code execution in some formats)
pub fn deserialize_json(json_str: &str) -> Result<serde_json::Value, String> {
    // TAINT SINK: serde_json::from_str with user-controlled JSON
    serde_json::from_str(json_str).map_err(|e| e.to_string())
}

/// TAINT SINK: Evaluate/interpret user string
pub fn eval_expression(expr: &str) -> Result<String, String> {
    // TAINT SINK: Simulated expression evaluation
    // In real code this might use a scripting engine
    println!("[EVAL] Evaluating: {}", expr);
    Ok(format!("Result of {}", expr))
}

// ============================================================================
// TEMPLATE/RENDER SINKS (potential for injection)
// ============================================================================

/// TAINT SINK: Render template with user data
pub fn render_template(template: &str, data: &std::collections::HashMap<String, String>) -> String {
    // TAINT SINK: Template injection if template is user-controlled
    let mut result = template.to_string();
    for (key, value) in data {
        result = result.replace(&format!("{{{{{}}}}}", key), value);
    }
    result
}

/// TAINT SINK: Build HTML with user content (XSS)
pub fn build_html(title: &str, body: &str) -> String {
    // TAINT SINK: HTML injection / XSS
    format!(
        "<!DOCTYPE html><html><head><title>{}</title></head><body>{}</body></html>",
        title, body
    )
}

// ============================================================================
// REGEX SINKS (ReDoS)
// ============================================================================

/// TAINT SINK: Compile user-provided regex (ReDoS risk)
pub fn compile_regex(pattern: &str) -> Result<regex::Regex, String> {
    // TAINT SINK: regex::Regex::new with user-controlled pattern
    // Can cause catastrophic backtracking (ReDoS)
    regex::Regex::new(pattern).map_err(|e| e.to_string())
}

// Note: regex crate not in dependencies, this is illustrative
// In real code, add regex = "1" to Cargo.toml

mod regex {
    pub struct Regex;
    impl Regex {
        pub fn new(_pattern: &str) -> Result<Self, RegexError> {
            Ok(Regex)
        }
    }
    pub struct RegexError;
    impl std::fmt::Display for RegexError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "regex error")
        }
    }
}
