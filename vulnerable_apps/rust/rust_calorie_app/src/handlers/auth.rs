//! Authentication handlers - registration, login, profile management.
//!
//! TAINT SOURCE: All request bodies and parameters originate from HTTP clients.

use actix_web::{web, HttpResponse};

use crate::config::AppConfig;
use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::{
    ApiResponse, ChangePasswordRequest, CreateUserRequest, LoginRequest, UpdateUserRequest,
};
use crate::services::AuthService;
use crate::validation::Validators;

/// POST /api/auth/register - Register a new user
///
/// TAINT SOURCE: web::Json<CreateUserRequest> contains user-provided data
pub async fn register(
    pool: web::Data<DbPool>,
    body: web::Json<CreateUserRequest>,  // TAINT SOURCE: HTTP request body
) -> Result<HttpResponse, AppError> {
    // TAINT: body.into_inner() extracts tainted data
    let request = body.into_inner();

    // TAINT TRANSFORM: Validate input (may reject invalid data)
    Validators::validate_user_registration(&request)?;

    // TAINT FLOW: request -> AuthService -> UserRepository -> SQL
    let user = AuthService::register(&pool, request).await?;

    Ok(HttpResponse::Created().json(ApiResponse::success_with_message(user, "Registration successful")))
}

/// POST /api/auth/login - Authenticate user
///
/// TAINT SOURCE: web::Json<LoginRequest> contains credentials
pub async fn login(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    body: web::Json<LoginRequest>,  // TAINT SOURCE: HTTP request body
) -> Result<HttpResponse, AppError> {
    // TAINT: Credentials from HTTP
    let request = body.into_inner();

    // TAINT TRANSFORM: Validate input
    Validators::validate_login(&request)?;

    // TAINT FLOW: request -> AuthService -> UserRepository -> SQL
    let response = AuthService::login(&pool, &config, request).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

/// GET /api/auth/me - Get current user profile
///
/// Uses authenticated user ID from middleware (originally from JWT token)
pub async fn get_profile(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,  // TAINT SOURCE: From JWT token (HTTP header)
) -> Result<HttpResponse, AppError> {
    // TAINT: user_id extracted from JWT token
    let user = AuthService::get_user_by_id(&pool, &user_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(crate::models::UserResponse::from(user))))
}

/// PUT /api/auth/profile - Update user profile
///
/// TAINT SOURCE: Both user_id (from token) and body (from request)
pub async fn update_profile(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,  // TAINT SOURCE: From JWT
    body: web::Json<UpdateUserRequest>,  // TAINT SOURCE: HTTP body
) -> Result<HttpResponse, AppError> {
    let request = body.into_inner();

    // TAINT FLOW: request -> UserRepository -> SQL UPDATE
    let user = crate::repositories::UserRepository::update(&pool, &user_id, &request).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(crate::models::UserResponse::from(user))))
}

/// POST /api/auth/change-password - Change password
///
/// TAINT SOURCE: Both passwords from HTTP body
pub async fn change_password(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,  // TAINT SOURCE: From JWT
    body: web::Json<ChangePasswordRequest>,  // TAINT SOURCE: HTTP body
) -> Result<HttpResponse, AppError> {
    let request = body.into_inner();

    // TAINT TRANSFORM: Validate
    Validators::validate_password_change(&request)?;

    // TAINT FLOW: passwords -> AuthService -> hash -> UserRepository -> SQL
    AuthService::change_password(
        &pool,
        &user_id,
        &request.current_password,
        &request.new_password,
    )
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::<()>::success_with_message((), "Password changed successfully")))
}

/// DELETE /api/auth/account - Delete user account
pub async fn delete_account(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,  // TAINT SOURCE: From JWT
) -> Result<HttpResponse, AppError> {
    // TAINT FLOW: user_id -> UserRepository -> SQL DELETE
    let deleted = crate::repositories::UserRepository::delete(&pool, &user_id).await?;

    if deleted {
        Ok(HttpResponse::Ok().json(ApiResponse::<()>::success_with_message((), "Account deleted")))
    } else {
        Err(AppError::NotFound("Account not found".to_string()))
    }
}
