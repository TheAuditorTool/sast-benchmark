//! Middleware for job processing pipeline
//!
//! Provides hooks for job lifecycle events.

use async_trait::async_trait;
use std::sync::Arc;
use std::time::Instant;

use jobqueue_core::{Job, Result};

/// Middleware trait for job processing
#[async_trait]
pub trait Middleware: Send + Sync {
    /// Called before job execution
    async fn before(&self, job: &mut Job) -> Result<()> {
        Ok(())
    }

    /// Called after job execution (success or failure)
    async fn after(&self, job: &Job, result: &Result<serde_json::Value>) -> Result<()> {
        Ok(())
    }

    /// Middleware name
    fn name(&self) -> &str {
        "unnamed"
    }
}

/// Stack of middleware
pub struct MiddlewareStack {
    middlewares: Vec<Arc<dyn Middleware>>,
}

impl MiddlewareStack {
    /// Create an empty middleware stack
    pub fn new() -> Self {
        Self {
            middlewares: Vec::new(),
        }
    }

    /// Add middleware to the stack
    pub fn add(&mut self, middleware: impl Middleware + 'static) {
        self.middlewares.push(Arc::new(middleware));
    }

    /// Run before hooks
    pub async fn run_before(&self, job: &mut Job) -> Result<()> {
        for middleware in &self.middlewares {
            middleware.before(job).await?;
        }
        Ok(())
    }

    /// Run after hooks (in reverse order)
    pub async fn run_after(&self, job: &Job, result: &Result<serde_json::Value>) -> Result<()> {
        for middleware in self.middlewares.iter().rev() {
            // VULNERABILITY: Ignores errors from after hooks
            let _ = middleware.after(job, result).await;
        }
        Ok(())
    }
}

impl Default for MiddlewareStack {
    fn default() -> Self {
        Self::new()
    }
}

/// Logging middleware
pub struct LoggingMiddleware {
    log_payload: bool,
}

impl LoggingMiddleware {
    pub fn new() -> Self {
        Self { log_payload: false }
    }

    pub fn with_payload(mut self) -> Self {
        self.log_payload = true;
        self
    }
}

impl Default for LoggingMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Middleware for LoggingMiddleware {
    async fn before(&self, job: &mut Job) -> Result<()> {
        tracing::info!(
            job_id = %job.id,
            job_type = %job.payload.job_type,
            queue = %job.queue,
            attempt = job.attempt,
            "Job starting"
        );

        if self.log_payload {
            // VULNERABILITY: Logs potentially sensitive payload data
            tracing::debug!(
                payload = ?job.payload.data,
                "Job payload"
            );
        }

        Ok(())
    }

    async fn after(&self, job: &Job, result: &Result<serde_json::Value>) -> Result<()> {
        match result {
            Ok(data) => {
                tracing::info!(
                    job_id = %job.id,
                    "Job completed successfully"
                );
                if self.log_payload {
                    // VULNERABILITY: Logs result data
                    tracing::debug!(result = ?data, "Job result");
                }
            }
            Err(e) => {
                tracing::error!(
                    job_id = %job.id,
                    error = %e,
                    "Job failed"
                );
            }
        }

        Ok(())
    }

    fn name(&self) -> &str {
        "logging"
    }
}

/// Timing middleware
pub struct TimingMiddleware {
    start_times: std::sync::Mutex<std::collections::HashMap<String, Instant>>,
}

impl TimingMiddleware {
    pub fn new() -> Self {
        Self {
            start_times: std::sync::Mutex::new(std::collections::HashMap::new()),
        }
    }
}

impl Default for TimingMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Middleware for TimingMiddleware {
    async fn before(&self, job: &mut Job) -> Result<()> {
        let mut times = self.start_times.lock().unwrap();
        times.insert(job.id.to_string(), Instant::now());
        Ok(())
    }

    async fn after(&self, job: &Job, _result: &Result<serde_json::Value>) -> Result<()> {
        let mut times = self.start_times.lock().unwrap();
        if let Some(start) = times.remove(&job.id.to_string()) {
            let duration = start.elapsed();
            tracing::info!(
                job_id = %job.id,
                duration_ms = duration.as_millis() as u64,
                "Job duration"
            );
        }
        Ok(())
    }

    fn name(&self) -> &str {
        "timing"
    }
}

/// Rate limiting middleware
pub struct RateLimitMiddleware {
    max_per_second: u32,
    window: std::sync::Mutex<RateLimitWindow>,
}

struct RateLimitWindow {
    count: u32,
    window_start: Instant,
}

impl RateLimitMiddleware {
    pub fn new(max_per_second: u32) -> Self {
        Self {
            max_per_second,
            window: std::sync::Mutex::new(RateLimitWindow {
                count: 0,
                window_start: Instant::now(),
            }),
        }
    }
}

#[async_trait]
impl Middleware for RateLimitMiddleware {
    async fn before(&self, _job: &mut Job) -> Result<()> {
        let mut window = self.window.lock().unwrap();

        // Reset window if expired
        if window.window_start.elapsed().as_secs() >= 1 {
            window.count = 0;
            window.window_start = Instant::now();
        }

        // Check rate limit
        if window.count >= self.max_per_second {
            return Err(jobqueue_core::JobQueueError::RateLimitExceeded {
                limit: self.max_per_second,
            });
        }

        window.count += 1;
        Ok(())
    }

    fn name(&self) -> &str {
        "rate_limit"
    }
}

/// Validation middleware
pub struct ValidationMiddleware {
    max_payload_size: usize,
    required_metadata: Vec<String>,
}

impl ValidationMiddleware {
    pub fn new() -> Self {
        Self {
            max_payload_size: 1024 * 1024, // 1MB
            required_metadata: Vec::new(),
        }
    }

    pub fn max_payload_size(mut self, size: usize) -> Self {
        self.max_payload_size = size;
        self
    }

    pub fn require_metadata(mut self, key: impl Into<String>) -> Self {
        self.required_metadata.push(key.into());
        self
    }
}

impl Default for ValidationMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Middleware for ValidationMiddleware {
    async fn before(&self, job: &mut Job) -> Result<()> {
        // Check payload size
        let payload_str = serde_json::to_string(&job.payload.data)
            .map_err(|e| jobqueue_core::JobQueueError::SerializationError(e))?;

        if payload_str.len() > self.max_payload_size {
            return Err(jobqueue_core::JobQueueError::ConfigError(
                format!("Payload too large: {} > {}", payload_str.len(), self.max_payload_size)
            ));
        }

        // Check required metadata
        for key in &self.required_metadata {
            if !job.payload.metadata.contains_key(key) {
                return Err(jobqueue_core::JobQueueError::ConfigError(
                    format!("Missing required metadata: {}", key)
                ));
            }
        }

        Ok(())
    }

    fn name(&self) -> &str {
        "validation"
    }
}

/// Retry policy middleware
pub struct RetryMiddleware {
    max_retries: u32,
}

impl RetryMiddleware {
    pub fn new(max_retries: u32) -> Self {
        Self { max_retries }
    }
}

#[async_trait]
impl Middleware for RetryMiddleware {
    async fn before(&self, job: &mut Job) -> Result<()> {
        if job.attempt > self.max_retries {
            return Err(jobqueue_core::JobQueueError::Internal(
                "Max retries exceeded".into()
            ));
        }
        Ok(())
    }

    fn name(&self) -> &str {
        "retry"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jobqueue_core::Job;

    #[tokio::test]
    async fn test_middleware_stack() {
        let mut stack = MiddlewareStack::new();
        stack.add(LoggingMiddleware::new());
        stack.add(TimingMiddleware::new());

        let mut job = Job::builder("test").build();

        assert!(stack.run_before(&mut job).await.is_ok());
    }

    #[tokio::test]
    async fn test_rate_limit_middleware() {
        let middleware = RateLimitMiddleware::new(2);
        let mut job = Job::builder("test").build();

        // First two should pass
        assert!(middleware.before(&mut job).await.is_ok());
        assert!(middleware.before(&mut job).await.is_ok());

        // Third should fail
        assert!(middleware.before(&mut job).await.is_err());
    }
}
