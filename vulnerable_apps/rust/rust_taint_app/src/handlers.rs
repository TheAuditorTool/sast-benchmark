//! HTTP handlers demonstrating taint sources from web framework inputs.
//!
//! All extractors (Json, Path, Query, Form) are TAINT SOURCES.

use actix_web::{web, HttpRequest, HttpResponse, Responder};
use crate::models::{
    ApiResponse, CommandRequest, FileRequest, ProxyRequest, ShellCommandRequest,
    User, UserInput, UserSearchQuery,
};
use crate::{commands, database, files, network, memory_ops, AppState};

/// TAINT SOURCE: web::Json extractor
/// GET /users - List all users
pub async fn get_users(
    state: web::Data<AppState>,
) -> impl Responder {
    match database::get_all_users(&state.db_pool).await {
        Ok(users) => HttpResponse::Ok().json(ApiResponse::success(users)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<Vec<User>>::error(&e.to_string())),
    }
}

/// TAINT SOURCE: web::Path extractor
/// GET /users/{id} - Get user by ID
pub async fn get_user_by_id(
    state: web::Data<AppState>,
    path: web::Path<i64>,  // TAINT SOURCE
) -> impl Responder {
    let user_id = path.into_inner();

    // TAINT FLOW: path param -> SQL query
    match database::get_user_by_id(&state.db_pool, user_id).await {
        Ok(Some(user)) => HttpResponse::Ok().json(ApiResponse::success(user)),
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::<User>::error("User not found")),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<User>::error(&e.to_string())),
    }
}

/// TAINT SOURCE: web::Json extractor
/// POST /users - Create a new user
pub async fn create_user(
    state: web::Data<AppState>,
    body: web::Json<UserInput>,  // TAINT SOURCE
) -> impl Responder {
    let input = body.into_inner();

    // TAINT FLOW: JSON body -> SQL insert
    match database::create_user(&state.db_pool, &input).await {
        Ok(user) => HttpResponse::Created().json(ApiResponse::success(user)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<User>::error(&e.to_string())),
    }
}

/// TAINT SOURCE: web::Query extractor
/// GET /users/search - Search users with query params
pub async fn search_users(
    state: web::Data<AppState>,
    query: web::Query<UserSearchQuery>,  // TAINT SOURCE
) -> impl Responder {
    let search_params = query.into_inner();

    // TAINT FLOW: query params -> SQL query (VULNERABLE!)
    match database::search_users_dynamic(&state.db_pool, &search_params).await {
        Ok(users) => HttpResponse::Ok().json(ApiResponse::success(users)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<Vec<User>>::error(&e.to_string())),
    }
}

/// TAINT SOURCE: web::Json extractor
/// POST /admin/exec - Execute a command (DANGEROUS!)
pub async fn execute_command(
    body: web::Json<CommandRequest>,  // TAINT SOURCE
    req: HttpRequest,
) -> impl Responder {
    let cmd_req = body.into_inner();

    // TAINT SOURCE: HttpRequest::match_info
    let _info = req.match_info();

    // TAINT SOURCE: HttpRequest::query_string
    let _query = req.query_string();

    // TAINT FLOW: JSON body -> Command execution (VULNERABLE!)
    match commands::execute_command(&cmd_req.command, &cmd_req.args) {
        Ok(output) => HttpResponse::Ok().json(ApiResponse::success(output)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string())),
    }
}

/// TAINT SOURCE: web::Json extractor
/// POST /admin/shell - Run shell command (VERY DANGEROUS!)
pub async fn run_shell_command(
    body: web::Json<ShellCommandRequest>,  // TAINT SOURCE
) -> impl Responder {
    let shell_req = body.into_inner();

    // TAINT FLOW: JSON body -> Shell execution (CRITICAL VULNERABILITY!)
    match commands::execute_shell_command(&shell_req.shell_command) {
        Ok(output) => HttpResponse::Ok().json(ApiResponse::success(output)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string())),
    }
}

/// TAINT SOURCE: web::Query extractor
/// GET /files/read - Read a file (path traversal risk!)
pub async fn read_file_handler(
    query: web::Query<FileRequest>,  // TAINT SOURCE
) -> impl Responder {
    let file_req = query.into_inner();

    // TAINT FLOW: query param -> file read (PATH TRAVERSAL!)
    match files::read_file(&file_req.path) {
        Ok(content) => HttpResponse::Ok().json(ApiResponse::success(content)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string())),
    }
}

/// TAINT SOURCE: web::Json extractor
/// POST /files/write - Write to a file (path traversal risk!)
pub async fn write_file_handler(
    body: web::Json<FileRequest>,  // TAINT SOURCE
) -> impl Responder {
    let file_req = body.into_inner();

    let content = file_req.content.unwrap_or_default();

    // TAINT FLOW: JSON body -> file write (PATH TRAVERSAL!)
    match files::write_file(&file_req.path, &content) {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::success("File written")),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string())),
    }
}

/// TAINT SOURCE: web::Query extractor
/// GET /proxy - Proxy a request to external URL (SSRF risk!)
pub async fn proxy_request(
    query: web::Query<ProxyRequest>,  // TAINT SOURCE
) -> impl Responder {
    let proxy_req = query.into_inner();

    // TAINT FLOW: query param -> HTTP request (SSRF!)
    match network::fetch_url(&proxy_req.url).await {
        Ok(response) => HttpResponse::Ok().json(ApiResponse::success(response)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string())),
    }
}

/// TAINT SOURCE: web::Json extractor
/// POST /fetch - Fetch URL with custom options (SSRF risk!)
pub async fn fetch_url(
    body: web::Json<ProxyRequest>,  // TAINT SOURCE
) -> impl Responder {
    let proxy_req = body.into_inner();

    // TAINT FLOW: JSON body -> HTTP request (SSRF!)
    match network::fetch_url_with_options(&proxy_req).await {
        Ok(response) => HttpResponse::Ok().json(ApiResponse::success(response)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string())),
    }
}

/// TAINT SOURCE: web::Form extractor
/// Demonstrates form input handling
pub async fn handle_form(
    form: web::Form<UserInput>,  // TAINT SOURCE
) -> impl Responder {
    let input = form.into_inner();

    // Form data is tainted
    HttpResponse::Ok().json(ApiResponse::success(format!("Received: {}", input.username)))
}

/// TAINT SOURCE: web::Data (app state can contain tainted data)
pub async fn get_app_data(
    data: web::Data<AppState>,  // TAINT SOURCE (state)
) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::success(format!("Debug mode: {}", data.config.debug_mode)))
}

/// POST /unsafe/transmute - Demonstrates unsafe memory operations
pub async fn unsafe_transmute_handler(
    body: web::Json<Vec<u8>>,  // TAINT SOURCE
) -> impl Responder {
    let data = body.into_inner();

    // TAINT FLOW: JSON body -> unsafe memory operation
    match memory_ops::dangerous_transmute(&data) {
        Ok(result) => HttpResponse::Ok().json(ApiResponse::success(result)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e)),
    }
}
