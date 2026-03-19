//! Additional handler utilities

use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;

/// Standard API response wrapper
#[derive(serde::Serialize)]
pub struct ApiResponse<T: serde::Serialize> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
}

impl<T: serde::Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            error_code: None,
        }
    }

    pub fn error(message: impl Into<String>) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            data: None,
            error: Some(message.into()),
            error_code: None,
        }
    }

    pub fn with_error_code(mut self, code: impl Into<String>) -> Self {
        self.error_code = Some(code.into());
        self
    }
}

/// Pagination info
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Pagination {
    pub page: usize,
    pub per_page: usize,
    pub total: usize,
    pub total_pages: usize,
}

impl Pagination {
    pub fn new(page: usize, per_page: usize, total: usize) -> Self {
        let total_pages = (total + per_page - 1) / per_page;
        Self {
            page,
            per_page,
            total,
            total_pages,
        }
    }

    pub fn offset(&self) -> usize {
        (self.page.saturating_sub(1)) * self.per_page
    }

    pub fn has_next(&self) -> bool {
        self.page < self.total_pages
    }

    pub fn has_prev(&self) -> bool {
        self.page > 1
    }
}

/// Paginated response
#[derive(serde::Serialize)]
pub struct PaginatedResponse<T: serde::Serialize> {
    pub items: Vec<T>,
    pub pagination: Pagination,
}

impl<T: serde::Serialize> PaginatedResponse<T> {
    pub fn new(items: Vec<T>, pagination: Pagination) -> Self {
        Self { items, pagination }
    }
}

/// Extract client IP from request
pub fn extract_client_ip(headers: &axum::http::HeaderMap) -> Option<String> {
    // Try X-Forwarded-For first (can be spoofed!)
    // VULNERABILITY: X-Forwarded-For can be spoofed
    if let Some(forwarded) = headers.get("x-forwarded-for") {
        if let Ok(value) = forwarded.to_str() {
            return Some(value.split(',').next()?.trim().to_string());
        }
    }

    // Try X-Real-IP
    if let Some(real_ip) = headers.get("x-real-ip") {
        if let Ok(value) = real_ip.to_str() {
            return Some(value.to_string());
        }
    }

    None
}

/// Validate content type
pub fn validate_content_type(headers: &axum::http::HeaderMap, expected: &str) -> bool {
    headers
        .get(axum::http::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .map(|v| v.starts_with(expected))
        .unwrap_or(false)
}

/// Rate limit key generator
pub fn rate_limit_key(ip: &str, endpoint: &str) -> String {
    // VULNERABILITY: Using MD5 for key generation
    let input = format!("{}:{}", ip, endpoint);
    format!("{:x}", md5::compute(input))
}

// MD5 stub (would use actual md5 crate)
mod md5 {
    pub fn compute(input: String) -> impl std::fmt::LowerHex {
        // Fake implementation - just returns length
        input.len() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination() {
        let pagination = Pagination::new(1, 10, 95);

        assert_eq!(pagination.total_pages, 10);
        assert_eq!(pagination.offset(), 0);
        assert!(pagination.has_next());
        assert!(!pagination.has_prev());
    }

    #[test]
    fn test_api_response() {
        let response = ApiResponse::success(vec![1, 2, 3]);
        assert!(response.success);
        assert!(response.data.is_some());

        let error_response: ApiResponse<()> = ApiResponse::error("Something went wrong");
        assert!(!error_response.success);
        assert!(error_response.error.is_some());
    }
}
