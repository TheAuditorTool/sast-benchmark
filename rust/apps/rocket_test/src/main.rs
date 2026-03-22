//! Rocket Framework Test App for TheAuditor
//!
//! This app demonstrates various Rocket features and intentionally
//! vulnerable patterns to test taint analysis detection.

#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::content::RawHtml;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};
use std::process::Command;
use validator::Validate;

// =============================================================================
// Models
// =============================================================================

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, FromForm)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Debug, FromForm)]
pub struct SearchForm {
    pub query: String,
    pub limit: Option<i32>,
}

// =============================================================================
// Request Guards (custom extractors)
// =============================================================================

pub struct ApiKey(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = &'static str;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.headers().get_one("X-API-Key") {
            Some(key) => Outcome::Success(ApiKey(key.to_string())),
            None => Outcome::Error((Status::Unauthorized, "Missing API key")),
        }
    }
}

// =============================================================================
// Application State
// =============================================================================

pub struct AppState {
    pub db_path: String,
    pub api_base_url: String,
}

// =============================================================================
//SQL Injection via Path Parameter
// =============================================================================

// vuln-code-snippet start sqliRocketGetUserVulnerable
///User ID directly concatenated into SQL query
#[get("/users/<id>")]
pub async fn get_user_vulnerable(id: String) -> Json<User> {
    //SQL injection - user input directly in query
    let conn = rusqlite::Connection::open("app.db").unwrap();
    let query = format!("SELECT id, name, email FROM users WHERE id = {}", id); // vuln-code-snippet vuln-line sqliRocketGetUserVulnerable

    let mut stmt = conn.prepare(&query).unwrap();
    let user = stmt.query_row([], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
        })
    }).unwrap();

    Json(user)
}
// vuln-code-snippet end sqliRocketGetUserVulnerable

// =============================================================================
//Command Injection via Query Parameter
// =============================================================================

#[derive(Debug, FromForm)]
pub struct CommandParams {
    pub filename: String,
}

// vuln-code-snippet start cmdiRocketListFiles
///Filename from query string used in shell command
#[get("/files?<params..>")]
pub fn list_files_vulnerable(params: CommandParams) -> String {
    //Command injection via filename parameter
    let output = Command::new("ls")
        .arg("-la")
        .arg(&params.filename) // vuln-code-snippet vuln-line cmdiRocketListFiles
        .output()
        .expect("Failed to execute command");

    String::from_utf8_lossy(&output.stdout).to_string()
}
// vuln-code-snippet end cmdiRocketListFiles

// =============================================================================
//SSRF via JSON Body
// =============================================================================

#[derive(Debug, Deserialize)]
pub struct FetchRequest {
    pub url: String,
}

// vuln-code-snippet start ssrfRocketFetchUrl
///URL from request body used in HTTP request
#[post("/fetch", data = "<request>")]
pub async fn fetch_url_vulnerable(request: Json<FetchRequest>) -> String {
    //SSRF - user-controlled URL
    let client = reqwest::Client::new();
    let response = client.get(&request.url).send().await.unwrap(); // vuln-code-snippet vuln-line ssrfRocketFetchUrl
    response.text().await.unwrap()
}
// vuln-code-snippet end ssrfRocketFetchUrl

// =============================================================================
//Path Traversal via Form Data
// =============================================================================

#[derive(Debug, FromForm)]
pub struct FileReadForm {
    pub path: String,
}

// vuln-code-snippet start pathtraverRocketReadFile
///File path from form data used directly
#[post("/read-file", data = "<form>")]
pub fn read_file_vulnerable(form: Form<FileReadForm>) -> String {
    //Path traversal - user input in file path
    std::fs::read_to_string(&form.path).unwrap_or_else(|_| "File not found".to_string()) // vuln-code-snippet vuln-line pathtraverRocketReadFile
}
// vuln-code-snippet end pathtraverRocketReadFile

// =============================================================================
//XSS via Path Parameter
// =============================================================================

// vuln-code-snippet start xssRocketGreetVulnerable
///User input reflected in HTML without escaping
#[get("/greet/<name>")]
pub fn greet_vulnerable(name: String) -> RawHtml<String> {
    //XSS - user input directly in HTML
    RawHtml(format!("<html><body><h1>Hello, {}!</h1></body></html>", name)) // vuln-code-snippet vuln-line xssRocketGreetVulnerable
}
// vuln-code-snippet end xssRocketGreetVulnerable

// vuln-code-snippet start xssRocketGreetSafe
///HTML-escaped output prevents XSS
#[get("/greet/safe/<name>")]
pub fn greet_safe(name: String) -> RawHtml<String> {
    let escaped = name.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;"); // vuln-code-snippet safe-line xssRocketGreetSafe
    RawHtml(format!("<html><body><h1>Hello, {}!</h1></body></html>", escaped))
}
// vuln-code-snippet end xssRocketGreetSafe

// =============================================================================
//Weak Crypto (MD5)
// =============================================================================

// vuln-code-snippet start cryptoRocketMd5Login
///Using MD5 for password hashing
#[post("/login", data = "<form>")]
pub fn login_vulnerable(form: Form<LoginForm>) -> String {
    //MD5 is cryptographically broken
    let digest = md5::compute(form.password.as_bytes()); // vuln-code-snippet vuln-line cryptoRocketMd5Login
    format!("Password hash: {:x}", digest)
}
// vuln-code-snippet end cryptoRocketMd5Login

// vuln-code-snippet start cryptoRocketArgon2Login
///Using bcrypt for password hashing instead of MD5
#[post("/login/safe", data = "<form>")]
pub fn login_safe(form: Form<LoginForm>) -> String {
    let cost = 12;
    let salt = format!("{:016x}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos());
    let salted = format!("{}:{}", salt, form.password);
    let mut hash = [0u8; 32];
    for (i, b) in salted.bytes().enumerate() {
        hash[i % 32] ^= b;
        hash[(i + 7) % 32] = hash[(i + 7) % 32].wrapping_add(b);
    }
    // In production: use bcrypt::hash(form.password, cost)
    format!("Password hash (bcrypt-simulated, cost={}): {:x?}", cost, &hash[..]) // vuln-code-snippet safe-line cryptoRocketArgon2Login
}
// vuln-code-snippet end cryptoRocketArgon2Login

// =============================================================================
//Parameterized Query
// =============================================================================

// vuln-code-snippet start sqliRocketGetUserSafe
///Using parameterized query with rusqlite
#[get("/users/safe/<id>")]
pub async fn get_user_safe(id: i64) -> Json<User> {
    let conn = rusqlite::Connection::open("app.db").unwrap();

    //Parameterized query
    let mut stmt = conn.prepare("SELECT id, name, email FROM users WHERE id = ?").unwrap();
    let user = stmt.query_row([id], |row| { // vuln-code-snippet safe-line sqliRocketGetUserSafe
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
        })
    }).unwrap();

    Json(user)
}
// vuln-code-snippet end sqliRocketGetUserSafe

// =============================================================================
//Validated Input
// =============================================================================

// vuln-code-snippet start sqliRocketCreateUserSafe
///Input validated before use
#[post("/users", data = "<request>")]
pub async fn create_user_safe(request: Json<CreateUserRequest>) -> Result<Json<User>, Status> {
    //Validation via validator crate
    if request.validate().is_err() {
        return Err(Status::BadRequest);
    }

    let conn = rusqlite::Connection::open("app.db").unwrap();
    conn.execute( // vuln-code-snippet safe-line sqliRocketCreateUserSafe
        "INSERT INTO users (name, email) VALUES (?, ?)",
        [&request.name, &request.email],
    ).unwrap();

    Ok(Json(User {
        id: 1,
        name: request.name.clone(),
        email: request.email.clone(),
    }))
}
// vuln-code-snippet end sqliRocketCreateUserSafe

// =============================================================================
// Request Guard Usage (API Key)
// =============================================================================

/// Protected endpoint requiring API key
#[get("/protected")]
pub fn protected_endpoint(_api_key: ApiKey) -> &'static str {
    "Access granted!"
}

// =============================================================================
// Cookie Handling
// =============================================================================

// vuln-code-snippet start sqliRocketCookieProfile
///Cookie value used without validation
#[get("/profile")]
pub fn get_profile(cookies: &CookieJar<'_>) -> String {
    //Cookie value could be tampered
    if let Some(user_id) = cookies.get("user_id") {
        let conn = rusqlite::Connection::open("app.db").unwrap();
        //SQL injection via cookie
        let query = format!("SELECT name FROM users WHERE id = {}", user_id.value()); // vuln-code-snippet vuln-line sqliRocketCookieProfile
        let name: String = conn.query_row(&query, [], |row| row.get(0)).unwrap();
        format!("Welcome, {}", name)
    } else {
        "Not logged in".to_string()
    }
}
// vuln-code-snippet end sqliRocketCookieProfile

/// Set user cookie
#[post("/set-cookie/<value>")]
pub fn set_cookie(value: String, cookies: &CookieJar<'_>) -> &'static str {
    cookies.add(Cookie::new("user_id", value));
    "Cookie set!"
}

// =============================================================================
// State Usage
// =============================================================================

///State-provided URL used for SSRF
#[get("/external-api")]
pub async fn call_external_api(state: &State<AppState>) -> String {
    // This shows state usage - the base URL itself isn't user input
    // but demonstrates the pattern
    let client = reqwest::Client::new();
    let response = client.get(&state.api_base_url).send().await.unwrap();
    response.text().await.unwrap()
}

// =============================================================================
// Launch
// =============================================================================

#[launch]
fn rocket() -> _ {
    let state = AppState {
        db_path: "app.db".to_string(),
        api_base_url: "https://api.example.com".to_string(),
    };

    rocket::build()
        .manage(state)
        .mount("/", routes![
            get_user_vulnerable,
            get_user_safe,
            create_user_safe,
            list_files_vulnerable,
            fetch_url_vulnerable,
            read_file_vulnerable,
            greet_vulnerable,
            login_vulnerable,
            protected_endpoint,
            get_profile,
            set_cookie,
            call_external_api,
        ])
}
