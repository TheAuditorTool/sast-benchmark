//! API error handling
//!
//! VULNERABILITY: Error messages may leak internal details

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

/// API error type
#[derive(Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub code: String,
    pub message: String,
    pub details: Option<String>,
}

impl ApiError {
    pub fn new(status: StatusCode, code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            status,
            code: code.into(),
            message: message.into(),
            details: None,
        }
    }

    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, "BAD_REQUEST", message)
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, "NOT_FOUND", message)
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, "UNAUTHORIZED", message)
    }

    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::new(StatusCode::FORBIDDEN, "FORBIDDEN", message)
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", message)
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self::new(StatusCode::CONFLICT, "CONFLICT", message)
    }

    pub fn rate_limited() -> Self {
        Self::new(StatusCode::TOO_MANY_REQUESTS, "RATE_LIMITED", "Too many requests")
    }

    pub fn validation(message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNPROCESSABLE_ENTITY, "VALIDATION_ERROR", message)
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    success: bool,
    error: ErrorBody,
}

#[derive(Serialize)]
struct ErrorBody {
    code: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        // VULNERABILITY: May include sensitive details in response
        // Also reflects user input which could lead to XSS if rendered in browser
        let body = ErrorResponse {
            success: false,
            error: ErrorBody {
                code: self.code,
                message: self.message,
                details: self.details,
            },
        };

        (self.status, Json(body)).into_response()
    }
}

/// Convert from jobqueue_core errors
impl From<jobqueue_core::JobQueueError> for ApiError {
    fn from(err: jobqueue_core::JobQueueError) -> Self {
        // VULNERABILITY: Leaks internal error details
        match &err {
            jobqueue_core::JobQueueError::JobNotFound { job_id } => {
                Self::not_found(format!("Job not found: {}", job_id))
            }
            jobqueue_core::JobQueueError::QueueNotFound { queue_name } => {
                Self::not_found(format!("Queue not found: {}", queue_name))
            }
            // vuln-code-snippet start infodisclosureJobqueueErrorSql
            jobqueue_core::JobQueueError::DatabaseError { message, query } => {
                // VULNERABILITY: Includes SQL query in error response
                let details = query.as_ref().map(|q| format!("Query: {}", q)); // vuln-code-snippet vuln-line infodisclosureJobqueueErrorSql
                Self::internal(message.clone()).with_details(details.unwrap_or_default())
            }
            // vuln-code-snippet end infodisclosureJobqueueErrorSql
            jobqueue_core::JobQueueError::Timeout { duration_ms } => {
                Self::internal(format!("Operation timed out after {}ms", duration_ms))
            }
            jobqueue_core::JobQueueError::PermissionDenied { action } => {
                Self::forbidden(format!("Permission denied: {}", action))
            }
            jobqueue_core::JobQueueError::RateLimitExceeded { limit } => {
                Self::rate_limited().with_details(format!("Limit: {}/s", limit))
            }
            _ => {
                // VULNERABILITY: Includes full error string
                Self::internal(err.to_string())
            }
        }
    }
}

/// Convert from database errors
impl From<jobqueue_db::DbError> for ApiError {
    fn from(err: jobqueue_db::DbError) -> Self {
        // VULNERABILITY: Exposes database errors
        Self::internal(format!("Database error: {}", err))
    }
}

/// Validation error with field-level details
#[derive(Debug, Serialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

impl ValidationError {
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
        }
    }
}

/// Collection of validation errors
#[derive(Debug)]
pub struct ValidationErrors(pub Vec<ValidationError>);

impl IntoResponse for ValidationErrors {
    fn into_response(self) -> Response {
        let body = serde_json::json!({
            "success": false,
            "error": {
                "code": "VALIDATION_ERROR",
                "message": "Validation failed",
                "fields": self.0
            }
        });

        (StatusCode::UNPROCESSABLE_ENTITY, Json(body)).into_response()
    }
}

/// Helper macro for creating errors
#[macro_export]
macro_rules! api_error {
    ($status:expr, $code:expr, $msg:expr) => {
        ApiError::new($status, $code, $msg)
    };
    ($status:expr, $code:expr, $msg:expr, $details:expr) => {
        ApiError::new($status, $code, $msg).with_details($details)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_error_creation() {
        let error = ApiError::not_found("Resource not found");
        assert_eq!(error.status, StatusCode::NOT_FOUND);
        assert_eq!(error.code, "NOT_FOUND");
    }

    #[test]
    fn test_error_with_details() {
        let error = ApiError::internal("Something went wrong")
            .with_details("Stack trace here");
        assert!(error.details.is_some());
    }

    #[test]
    fn test_validation_errors() {
        let errors = ValidationErrors(vec![
            ValidationError::new("email", "Invalid email format"),
            ValidationError::new("password", "Password too short"),
        ]);
        assert_eq!(errors.0.len(), 2);
    }
}
