//! Application errors - Centralized error handling.
//!
//! All errors flow through this module for consistent API responses.
//! Some errors contain tainted data (e.g., validation error messages).

use actix_web::{HttpResponse, ResponseError};
use std::fmt;

/// Application error types
#[derive(Debug)]
pub enum AppError {
    /// Validation error (400)
    /// TAINT: May contain user input in error message
    Validation(String),

    /// Authentication error (401)
    Unauthorized(String),

    /// Forbidden (403)
    Forbidden(String),

    /// Not found (404)
    NotFound(String),

    /// Conflict (409)
    Conflict(String),

    /// Internal server error (500)
    Internal(String),

    /// Database error (500)
    Database(String),

    /// BCrypt error (500)
    Bcrypt(String),

    /// JWT error (401)
    Jwt(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Validation(msg) => write!(f, "Validation error: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            AppError::Internal(msg) => write!(f, "Internal error: {}", msg),
            AppError::Database(msg) => write!(f, "Database error: {}", msg),
            AppError::Bcrypt(msg) => write!(f, "Password error: {}", msg),
            AppError::Jwt(msg) => write!(f, "Token error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (status, error_type, message) = match self {
            AppError::Validation(msg) => {
                (actix_web::http::StatusCode::BAD_REQUEST, "validation_error", msg.clone())
            }
            AppError::Unauthorized(msg) => {
                (actix_web::http::StatusCode::UNAUTHORIZED, "unauthorized", msg.clone())
            }
            AppError::Forbidden(msg) => {
                (actix_web::http::StatusCode::FORBIDDEN, "forbidden", msg.clone())
            }
            AppError::NotFound(msg) => {
                (actix_web::http::StatusCode::NOT_FOUND, "not_found", msg.clone())
            }
            AppError::Conflict(msg) => {
                (actix_web::http::StatusCode::CONFLICT, "conflict", msg.clone())
            }
            AppError::Internal(msg) => {
                // Don't expose internal error details in production
                let safe_msg = if cfg!(debug_assertions) {
                    msg.clone()
                } else {
                    "An internal error occurred".to_string()
                };
                (actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, "internal_error", safe_msg)
            }
            AppError::Database(msg) => {
                let safe_msg = if cfg!(debug_assertions) {
                    msg.clone()
                } else {
                    "A database error occurred".to_string()
                };
                (actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, "database_error", safe_msg)
            }
            AppError::Bcrypt(msg) => {
                let safe_msg = if cfg!(debug_assertions) {
                    msg.clone()
                } else {
                    "A password processing error occurred".to_string()
                };
                (actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, "password_error", safe_msg)
            }
            AppError::Jwt(msg) => {
                (actix_web::http::StatusCode::UNAUTHORIZED, "token_error", msg.clone())
            }
        };

        HttpResponse::build(status).json(serde_json::json!({
            "success": false,
            "error": {
                "type": error_type,
                "message": message
            }
        }))
    }
}

// From implementations for common error types
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound("Resource not found".to_string()),
            sqlx::Error::Database(db_err) => {
                // Check for unique constraint violations
                let msg = db_err.message();
                if msg.contains("UNIQUE constraint") {
                    AppError::Conflict("Resource already exists".to_string())
                } else {
                    AppError::Database(msg.to_string())
                }
            }
            _ => AppError::Database(err.to_string()),
        }
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(err: bcrypt::BcryptError) -> Self {
        AppError::Bcrypt(err.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        match err.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                AppError::Jwt("Token has expired".to_string())
            }
            jsonwebtoken::errors::ErrorKind::InvalidToken => {
                AppError::Jwt("Invalid token".to_string())
            }
            _ => AppError::Jwt("Token error".to_string()),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Internal(err.to_string())
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(errors: validator::ValidationErrors) -> Self {
        let messages: Vec<String> = errors
            .field_errors()
            .iter()
            .flat_map(|(field, errs)| {
                errs.iter().map(move |e| {
                    e.message
                        .clone()
                        .map(|m| m.to_string())
                        .unwrap_or_else(|| format!("Invalid {}", field))
                })
            })
            .collect();

        AppError::Validation(messages.join(", "))
    }
}
