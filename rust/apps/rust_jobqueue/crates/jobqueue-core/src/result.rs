//! Job result types and helpers
//!
//! This module provides types for representing job execution results.

use serde::{Deserialize, Serialize};
use std::fmt;

use crate::job::JobId;

/// Result of a job execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobResult {
    /// Job ID
    pub job_id: JobId,
    /// Whether the job succeeded
    pub success: bool,
    /// Result data (if successful)
    pub data: Option<serde_json::Value>,
    /// Error message (if failed)
    pub error: Option<String>,
    /// Execution duration in milliseconds
    pub duration_ms: u64,
    /// Attempt number (1-indexed)
    pub attempt: u32,
    /// Additional metadata
    #[serde(default)]
    pub metadata: std::collections::HashMap<String, String>,
}

impl JobResult {
    /// Create a successful result
    pub fn success(job_id: JobId, data: impl Serialize) -> Result<Self, serde_json::Error> {
        Ok(Self {
            job_id,
            success: true,
            data: Some(serde_json::to_value(data)?),
            error: None,
            duration_ms: 0,
            attempt: 1,
            metadata: std::collections::HashMap::new(),
        })
    }

    /// Create a successful result with raw JSON
    pub fn success_raw(job_id: JobId, data: serde_json::Value) -> Self {
        Self {
            job_id,
            success: true,
            data: Some(data),
            error: None,
            duration_ms: 0,
            attempt: 1,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Create a failure result
    pub fn failure(job_id: JobId, error: impl Into<String>) -> Self {
        Self {
            job_id,
            success: false,
            data: None,
            error: Some(error.into()),
            duration_ms: 0,
            attempt: 1,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Set duration
    pub fn with_duration(mut self, duration_ms: u64) -> Self {
        self.duration_ms = duration_ms;
        self
    }

    /// Set attempt number
    pub fn with_attempt(mut self, attempt: u32) -> Self {
        self.attempt = attempt;
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Get the result data as a specific type
    pub fn get_data<T: for<'de> Deserialize<'de>>(&self) -> Option<Result<T, serde_json::Error>> {
        self.data.as_ref().map(|v| serde_json::from_value(v.clone()))
    }

    /// Check if this result indicates a retryable failure
    pub fn is_retryable(&self) -> bool {
        if self.success {
            return false;
        }

        // Check for specific error patterns that indicate retryable failures
        if let Some(ref error) = self.error {
            let error_lower = error.to_lowercase();
            error_lower.contains("timeout")
                || error_lower.contains("connection")
                || error_lower.contains("temporary")
                || error_lower.contains("retry")
                || error_lower.contains("rate limit")
        } else {
            false
        }
    }
}

impl fmt::Display for JobResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.success {
            write!(f, "Job {} completed successfully in {}ms", self.job_id, self.duration_ms)
        } else {
            write!(
                f,
                "Job {} failed: {}",
                self.job_id,
                self.error.as_deref().unwrap_or("Unknown error")
            )
        }
    }
}

/// Batch result for multiple jobs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResult {
    /// Total jobs in batch
    pub total: usize,
    /// Successful jobs
    pub succeeded: usize,
    /// Failed jobs
    pub failed: usize,
    /// Individual results
    pub results: Vec<JobResult>,
    /// Total duration
    pub total_duration_ms: u64,
}

impl BatchResult {
    /// Create a new batch result
    pub fn new(results: Vec<JobResult>) -> Self {
        let total = results.len();
        let succeeded = results.iter().filter(|r| r.success).count();
        let failed = total - succeeded;
        let total_duration_ms = results.iter().map(|r| r.duration_ms).sum();

        Self {
            total,
            succeeded,
            failed,
            results,
            total_duration_ms,
        }
    }

    /// Get success rate (0.0 - 1.0)
    pub fn success_rate(&self) -> f64 {
        if self.total == 0 {
            1.0
        } else {
            self.succeeded as f64 / self.total as f64
        }
    }

    /// Check if all jobs succeeded
    pub fn all_succeeded(&self) -> bool {
        self.failed == 0
    }

    /// Get failed results only
    pub fn failures(&self) -> impl Iterator<Item = &JobResult> {
        self.results.iter().filter(|r| !r.success)
    }

    /// Get successful results only
    pub fn successes(&self) -> impl Iterator<Item = &JobResult> {
        self.results.iter().filter(|r| r.success)
    }
}

impl fmt::Display for BatchResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Batch: {}/{} succeeded ({:.1}%) in {}ms",
            self.succeeded,
            self.total,
            self.success_rate() * 100.0,
            self.total_duration_ms
        )
    }
}

/// Output collector for streaming results
#[derive(Debug, Default)]
pub struct OutputCollector {
    lines: Vec<String>,
    max_lines: Option<usize>,
}

impl OutputCollector {
    /// Create a new output collector
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with line limit
    pub fn with_limit(max_lines: usize) -> Self {
        Self {
            lines: Vec::new(),
            max_lines: Some(max_lines),
        }
    }

    /// Add a line of output
    pub fn push(&mut self, line: impl Into<String>) {
        if let Some(max) = self.max_lines {
            if self.lines.len() >= max {
                // VULNERABILITY: Just drops old lines without warning
                // Could lose important error messages
                self.lines.remove(0);
            }
        }
        self.lines.push(line.into());
    }

    /// Get all collected lines
    pub fn lines(&self) -> &[String] {
        &self.lines
    }

    /// Get as single string
    pub fn to_string_joined(&self, separator: &str) -> String {
        self.lines.join(separator)
    }

    /// Clear the collector
    pub fn clear(&mut self) {
        self.lines.clear();
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    /// Get line count
    pub fn len(&self) -> usize {
        self.lines.len()
    }
}

impl From<OutputCollector> for String {
    fn from(collector: OutputCollector) -> Self {
        collector.to_string_joined("\n")
    }
}

impl From<OutputCollector> for Vec<String> {
    fn from(collector: OutputCollector) -> Self {
        collector.lines
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_result_success() {
        let result = JobResult::success(
            JobId::new(),
            serde_json::json!({"message": "done"}),
        )
        .unwrap()
        .with_duration(100);

        assert!(result.success);
        assert_eq!(result.duration_ms, 100);
    }

    #[test]
    fn test_job_result_failure() {
        let result = JobResult::failure(JobId::new(), "Connection timeout");

        assert!(!result.success);
        assert!(result.is_retryable());
    }

    #[test]
    fn test_batch_result() {
        let results = vec![
            JobResult::success_raw(JobId::new(), serde_json::json!(null)),
            JobResult::success_raw(JobId::new(), serde_json::json!(null)),
            JobResult::failure(JobId::new(), "error"),
        ];

        let batch = BatchResult::new(results);

        assert_eq!(batch.total, 3);
        assert_eq!(batch.succeeded, 2);
        assert_eq!(batch.failed, 1);
        assert!(!batch.all_succeeded());
    }
}
