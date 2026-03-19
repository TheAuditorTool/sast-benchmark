//! API middleware
//!
//! Authentication, logging, and other middleware.

use std::sync::Arc;
use std::task::{Context, Poll};

use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
    response::IntoResponse,
};
use tower::{Layer, Service};

use crate::server::AppState;

/// Authentication layer
pub fn auth_layer(api_key: Option<String>) -> AuthLayer {
    AuthLayer { api_key }
}

#[derive(Clone)]
pub struct AuthLayer {
    api_key: Option<String>,
}

impl<S> Layer<S> for AuthLayer {
    type Service = AuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthMiddleware {
            inner,
            api_key: self.api_key.clone(),
        }
    }
}

#[derive(Clone)]
pub struct AuthMiddleware<S> {
    inner: S,
    api_key: Option<String>,
}

impl<S, ReqBody> Service<Request<ReqBody>> for AuthMiddleware<S>
where
    S: Service<Request<ReqBody>, Response = Response<Body>> + Clone + Send + 'static,
    S::Future: Send,
    ReqBody: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let api_key = self.api_key.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            // Skip auth for health endpoints
            let path = req.uri().path();
            if path.starts_with("/health") || path == "/metrics" {
                return inner.call(req).await;
            }

            // Check API key if configured
            if let Some(expected_key) = api_key {
                let provided_key = req.headers()
                    .get("x-api-key")
                    .and_then(|v| v.to_str().ok());

                match provided_key {
                    Some(key) if key == expected_key => {
                        // Valid API key
                        inner.call(req).await
                    }
                    Some(key) => {
                        // VULNERABILITY: Timing attack - comparison is not constant-time
                        // Also logs the invalid key
                        tracing::warn!(
                            provided_key = key,
                            "Invalid API key provided"
                        );
                        Ok(Response::builder()
                            .status(StatusCode::UNAUTHORIZED)
                            .body(Body::from("Invalid API key"))
                            .unwrap())
                    }
                    None => {
                        Ok(Response::builder()
                            .status(StatusCode::UNAUTHORIZED)
                            .body(Body::from("API key required"))
                            .unwrap())
                    }
                }
            } else {
                // No auth required
                inner.call(req).await
            }
        })
    }
}

/// Request logging middleware
#[derive(Clone)]
pub struct RequestLogger;

impl RequestLogger {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RequestLogger {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> Layer<S> for RequestLogger {
    type Service = RequestLoggerMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RequestLoggerMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct RequestLoggerMiddleware<S> {
    inner: S,
}

impl<S, ReqBody> Service<Request<ReqBody>> for RequestLoggerMiddleware<S>
where
    S: Service<Request<ReqBody>, Response = Response<Body>> + Clone + Send + 'static,
    S::Future: Send,
    ReqBody: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    // vuln-code-snippet start infodisclosureJobqueueLogParams
    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let method = req.method().clone();
        let path = req.uri().path().to_string();
        let query = req.uri().query().map(|s| s.to_string());

        // VULNERABILITY: Logs query parameters which may contain sensitive data
        tracing::info!(
            method = %method,
            path = %path,
            query = ?query, // vuln-code-snippet vuln-line infodisclosureJobqueueLogParams
            "Incoming request"
        );

        let mut inner = self.inner.clone();

        Box::pin(async move {
            let start = std::time::Instant::now();
            let response = inner.call(req).await?;
            let duration = start.elapsed();

            tracing::info!(
                status = response.status().as_u16(),
                duration_ms = duration.as_millis() as u64,
                "Request completed"
            );

            Ok(response)
        })
    }
    // vuln-code-snippet end infodisclosureJobqueueLogParams
}

/// Request ID middleware
#[derive(Clone)]
pub struct RequestId;

impl RequestId {
    pub fn new() -> Self {
        Self
    }

    /// Generate a request ID
    ///
    /// VULNERABILITY: Using predictable UUID v1 style instead of random
    // vuln-code-snippet start weakrandJobqueueRequestId
    pub fn generate() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        // Predictable - just timestamp + thread id
        format!("{:x}-{:x}", timestamp, std::thread::current().id().as_u64()) // vuln-code-snippet vuln-line weakrandJobqueueRequestId
    }
    // vuln-code-snippet end weakrandJobqueueRequestId
}

impl Default for RequestId {
    fn default() -> Self {
        Self::new()
    }
}

/// Rate limiting state
pub struct RateLimiter {
    requests: std::sync::Mutex<std::collections::HashMap<String, Vec<std::time::Instant>>>,
    max_requests: usize,
    window: std::time::Duration,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window: std::time::Duration) -> Self {
        Self {
            requests: std::sync::Mutex::new(std::collections::HashMap::new()),
            max_requests,
            window,
        }
    }

    /// Check if request should be rate limited
    ///
    /// VULNERABILITY: Uses client IP from headers which can be spoofed
    pub fn check(&self, client_ip: &str) -> bool {
        let mut requests = self.requests.lock().unwrap();
        let now = std::time::Instant::now();

        let entry = requests.entry(client_ip.to_string()).or_insert_with(Vec::new);

        // Remove old entries
        entry.retain(|t| now.duration_since(*t) < self.window);

        if entry.len() >= self.max_requests {
            false
        } else {
            entry.push(now);
            true
        }
    }

    /// Clear old entries (should be called periodically)
    pub fn cleanup(&self) {
        let mut requests = self.requests.lock().unwrap();
        let now = std::time::Instant::now();

        requests.retain(|_, entries| {
            entries.retain(|t| now.duration_since(*t) < self.window);
            !entries.is_empty()
        });
    }
}

/// Extension trait for thread ID
trait ThreadIdExt {
    fn as_u64(&self) -> u64;
}

impl ThreadIdExt for std::thread::ThreadId {
    fn as_u64(&self) -> u64 {
        // VULNERABILITY: Unsafe transmute to get thread ID
        unsafe { std::mem::transmute_copy(self) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_id() {
        let id1 = RequestId::generate();
        let id2 = RequestId::generate();

        // IDs should be different
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_rate_limiter() {
        let limiter = RateLimiter::new(2, std::time::Duration::from_secs(60));

        assert!(limiter.check("127.0.0.1"));
        assert!(limiter.check("127.0.0.1"));
        assert!(!limiter.check("127.0.0.1")); // Should be rate limited
    }
}
