//! Warp Framework Test App with filter and handler patterns.

use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::process::Command;
use std::sync::Arc;
use validator::Validate;
use warp::http::StatusCode;
use warp::Filter;

// =============================================================================
// Models
// =============================================================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub limit: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct FetchRequest {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct FileRequest {
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct CommandRequest {
    pub cmd: String,
    pub args: Vec<String>,
}

// =============================================================================
// Application State
// =============================================================================

#[derive(Clone)]
pub struct AppState {
    pub db_path: String,
    pub api_base_url: String,
}

fn with_state(
    state: Arc<AppState>,
) -> impl Filter<Extract = (Arc<AppState>,), Error = Infallible> + Clone {
    warp::any().map(move || state.clone())
}

// =============================================================================
//SQL Injection via Path Parameter
// =============================================================================

// vuln-code-snippet start sqliWarpGetUser
///User ID directly concatenated into SQL query
async fn get_user_vulnerable(id: String) -> Result<impl warp::Reply, Infallible> {
    //SQL injection - user input directly in query
    let conn = rusqlite::Connection::open("app.db").unwrap();
    let query = format!("SELECT id, name, email FROM users WHERE id = {}", id); // vuln-code-snippet target-line sqliWarpGetUser

    let result = conn.query_row(&query, [], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
        })
    });

    match result {
        Ok(user) => Ok(warp::reply::json(&user)),
        Err(_) => Ok(warp::reply::json(&serde_json::json!({"error": "User not found"}))),
    }
}
// vuln-code-snippet end sqliWarpGetUser

// =============================================================================
//Command Injection via Query Parameter
// =============================================================================

// vuln-code-snippet start cmdiWarpSearchFiles
///Query parameter used in shell command
async fn search_files_vulnerable(query: SearchQuery) -> Result<impl warp::Reply, Infallible> {
    //Command injection via search query
    let output = Command::new("grep")
        .arg("-r")
        .arg(&query.q) // vuln-code-snippet target-line cmdiWarpSearchFiles
        .arg(".")
        .output()
        .expect("Failed to execute command");

    Ok(warp::reply::html(String::from_utf8_lossy(&output.stdout).to_string()))
}
// vuln-code-snippet end cmdiWarpSearchFiles

// =============================================================================
//SSRF via JSON Body
// =============================================================================

// vuln-code-snippet start ssrfWarpFetchUrl
///URL from request body used in HTTP request
async fn fetch_url_vulnerable(request: FetchRequest) -> Result<impl warp::Reply, Infallible> {
    //SSRF - user-controlled URL
    let client = reqwest::Client::new();
    let response = client.get(&request.url).send().await; // vuln-code-snippet target-line ssrfWarpFetchUrl

    match response {
        Ok(resp) => {
            let text = resp.text().await.unwrap_or_default();
            Ok(warp::reply::html(text))
        }
        Err(_) => Ok(warp::reply::html("Failed to fetch".to_string())),
    }
}
// vuln-code-snippet end ssrfWarpFetchUrl

// =============================================================================
//Path Traversal via JSON Body
// =============================================================================

// vuln-code-snippet start pathtraverWarpReadFile
///File path from JSON body used directly
async fn read_file_vulnerable(request: FileRequest) -> Result<impl warp::Reply, Infallible> {
    //Path traversal - user input in file path
    let content = std::fs::read_to_string(&request.path) // vuln-code-snippet target-line pathtraverWarpReadFile
        .unwrap_or_else(|_| "File not found".to_string());

    Ok(warp::reply::html(content))
}
// vuln-code-snippet end pathtraverWarpReadFile

// =============================================================================
//Command Execution via JSON Body
// =============================================================================

// vuln-code-snippet start cmdiWarpExecuteCommand
///Command and args from request body
async fn execute_command_vulnerable(request: CommandRequest) -> Result<impl warp::Reply, Infallible> {
    //Arbitrary command execution
    let mut cmd = Command::new(&request.cmd); // vuln-code-snippet target-line cmdiWarpExecuteCommand
    for arg in &request.args {
        cmd.arg(arg);
    }

    let output = cmd.output().expect("Failed to execute command");
    Ok(warp::reply::html(String::from_utf8_lossy(&output.stdout).to_string()))
}
// vuln-code-snippet end cmdiWarpExecuteCommand

// =============================================================================
//Weak Crypto (SHA-1)
// =============================================================================

// vuln-code-snippet start cryptoWarpSha1Hash
///Using SHA-1 for hashing
async fn hash_vulnerable(data: bytes::Bytes) -> Result<impl warp::Reply, Infallible> {
    use sha1::{Digest, Sha1};

    //SHA-1 is deprecated for security
    let mut hasher = Sha1::new(); // vuln-code-snippet target-line cryptoWarpSha1Hash
    hasher.update(&data);
    let result = hasher.finalize();

    Ok(warp::reply::html(format!("{:x}", result)))
}
// vuln-code-snippet end cryptoWarpSha1Hash

// vuln-code-snippet start cryptoWarpSha256Hash
///Using SHA-256 instead of SHA-1
async fn hash_safe(data: bytes::Bytes) -> Result<impl warp::Reply, Infallible> {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new(); // vuln-code-snippet target-line cryptoWarpSha256Hash
    hasher.update(&data);
    let result = hasher.finalize();
    Ok(warp::reply::html(format!("{:x}", result)))
}
// vuln-code-snippet end cryptoWarpSha256Hash

// =============================================================================
//Header Injection
// =============================================================================

// vuln-code-snippet start xssWarpEchoHeader
///Header value used without validation
async fn echo_header(header_value: String) -> Result<impl warp::Reply, Infallible> {
    //Header value reflected in response
    Ok(warp::reply::html(format!("Header value: {}", header_value))) // vuln-code-snippet target-line xssWarpEchoHeader
}
// vuln-code-snippet end xssWarpEchoHeader

// vuln-code-snippet start xssWarpEchoHeader2
///HTML-escaped header reflection
async fn echo_header_safe(header_value: String) -> Result<impl warp::Reply, Infallible> {
    let escaped = header_value.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;"); // vuln-code-snippet target-line xssWarpEchoHeader2
    Ok(warp::reply::html(format!("Header value: {}", escaped)))
}
// vuln-code-snippet end xssWarpEchoHeader2

// =============================================================================
//Cookie Value in SQL
// =============================================================================

// vuln-code-snippet start sqliWarpCookieProfile
///Cookie value used in SQL query
async fn profile_vulnerable(session: String) -> Result<impl warp::Reply, Infallible> {
    let conn = rusqlite::Connection::open("app.db").unwrap();

    //SQL injection via cookie value
    let query = format!("SELECT name FROM users WHERE session = '{}'", session); // vuln-code-snippet target-line sqliWarpCookieProfile
    let name: Result<String, _> = conn.query_row(&query, [], |row| row.get(0));

    match name {
        Ok(n) => Ok(warp::reply::html(format!("Welcome, {}", n))),
        Err(_) => Ok(warp::reply::html("Not logged in".to_string())),
    }
}
// vuln-code-snippet end sqliWarpCookieProfile

// =============================================================================
//Parameterized Query
// =============================================================================

// vuln-code-snippet start sqliWarpGetUser2
///Using parameterized query
async fn get_user_safe(id: i64) -> Result<impl warp::Reply, Infallible> {
    let conn = rusqlite::Connection::open("app.db").unwrap();

    //Parameterized query
    let result = conn.query_row(
        "SELECT id, name, email FROM users WHERE id = ?", // vuln-code-snippet target-line sqliWarpGetUser2
        [id],
        |row| {
            Ok(User {
                id: row.get(0)?,
                name: row.get(1)?,
                email: row.get(2)?,
            })
        },
    );

    match result {
        Ok(user) => Ok(warp::reply::json(&user)),
        Err(_) => Ok(warp::reply::json(&serde_json::json!({"error": "User not found"}))),
    }
}
// vuln-code-snippet end sqliWarpGetUser2

// =============================================================================
//Validated Input
// =============================================================================

// vuln-code-snippet start sqliWarpCreateUser
///Input validated before use
async fn create_user_safe(request: CreateUserRequest) -> Result<impl warp::Reply, warp::Rejection> {
    //Validation via validator crate
    if let Err(_) = request.validate() {
        return Ok(warp::reply::with_status(
            warp::reply::json(&serde_json::json!({"error": "Invalid input"})),
            StatusCode::BAD_REQUEST,
        ));
    }

    let conn = rusqlite::Connection::open("app.db").unwrap();
    conn.execute(
        "INSERT INTO users (name, email) VALUES (?, ?)",
        [&request.name, &request.email],
    ) // vuln-code-snippet target-line sqliWarpCreateUser
    .unwrap();

    Ok(warp::reply::with_status(
        warp::reply::json(&User {
            id: 1,
            name: request.name,
            email: request.email,
        }),
        StatusCode::CREATED,
    ))
}
// vuln-code-snippet end sqliWarpCreateUser

// =============================================================================
// Route Definitions
// =============================================================================

fn routes(
    state: Arc<AppState>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let get_user_vulnerable = warp::path!("users" / String)
        .and(warp::get())
        .and_then(get_user_vulnerable);

    let get_user_safe = warp::path!("users" / "safe" / i64)
        .and(warp::get())
        .and_then(get_user_safe);

    let search = warp::path("search")
        .and(warp::get())
        .and(warp::query::<SearchQuery>())
        .and_then(search_files_vulnerable);

    let fetch = warp::path("fetch")
        .and(warp::post())
        .and(warp::body::json::<FetchRequest>())
        .and_then(fetch_url_vulnerable);

    let read_file = warp::path("read-file")
        .and(warp::post())
        .and(warp::body::json::<FileRequest>())
        .and_then(read_file_vulnerable);

    let execute = warp::path("execute")
        .and(warp::post())
        .and(warp::body::json::<CommandRequest>())
        .and_then(execute_command_vulnerable);

    let hash = warp::path("hash")
        .and(warp::post())
        .and(warp::body::bytes())
        .and_then(hash_vulnerable);

    let header_echo = warp::path("echo-header")
        .and(warp::get())
        .and(warp::header::<String>("X-Custom-Header"))
        .and_then(echo_header);

    let profile = warp::path("profile")
        .and(warp::get())
        .and(warp::cookie::<String>("session"))
        .and_then(profile_vulnerable);

    let create_user = warp::path("users")
        .and(warp::post())
        .and(warp::body::json::<CreateUserRequest>())
        .and_then(create_user_safe);

    // Combine all routes
    get_user_vulnerable
        .or(get_user_safe)
        .or(search)
        .or(fetch)
        .or(read_file)
        .or(execute)
        .or(hash)
        .or(header_echo)
        .or(profile)
        .or(create_user)
}

// =============================================================================
// Main
// =============================================================================

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        db_path: "app.db".to_string(),
        api_base_url: "https://api.example.com".to_string(),
    });

    let routes = routes(state);

    println!("Warp server starting on http://127.0.0.1:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
