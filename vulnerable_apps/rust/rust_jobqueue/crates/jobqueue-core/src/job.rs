//! Job definitions and builder pattern
//!
//! This module contains the core Job type and related structures.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;

use crate::priority::Priority;
use crate::types::{QueueName, WorkerId, Timestamp};

/// Unique identifier for a job
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct JobId(pub String);

impl JobId {
    /// Generate a new random job ID
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    /// Create a job ID from an existing string
    pub fn from_string(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    /// Get the inner string value
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for JobId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for JobId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for JobId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for JobId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// Current state of a job
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobState {
    /// Job is waiting in queue
    Pending,
    /// Job is currently being processed
    Running,
    /// Job completed successfully
    Completed,
    /// Job failed and won't be retried
    Failed,
    /// Job failed but will be retried
    Retrying,
    /// Job was cancelled
    Cancelled,
    /// Job is scheduled for future execution
    Scheduled,
    /// Job is waiting for dependencies
    Blocked,
}

impl JobState {
    /// Check if this state is terminal
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Cancelled)
    }

    /// Check if this state allows transitions
    pub fn can_transition_to(&self, target: &JobState) -> bool {
        use JobState::*;
        match (self, target) {
            // From Pending
            (Pending, Running) => true,
            (Pending, Cancelled) => true,
            (Pending, Scheduled) => true,

            // From Scheduled
            (Scheduled, Pending) => true,
            (Scheduled, Cancelled) => true,

            // From Running
            (Running, Completed) => true,
            (Running, Failed) => true,
            (Running, Retrying) => true,
            (Running, Cancelled) => true,

            // From Retrying
            (Retrying, Pending) => true,
            (Retrying, Failed) => true,
            (Retrying, Cancelled) => true,

            // From Blocked
            (Blocked, Pending) => true,
            (Blocked, Cancelled) => true,

            // Terminal states can't transition
            (Completed | Failed | Cancelled, _) => false,

            _ => false,
        }
    }
}

impl Default for JobState {
    fn default() -> Self {
        Self::Pending
    }
}

/// Job payload - the actual work to be done
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobPayload {
    /// Type of job (used for routing to correct handler)
    pub job_type: String,
    /// Serialized job data
    pub data: serde_json::Value,
    /// Optional metadata
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

impl JobPayload {
    /// Create a new payload
    pub fn new(job_type: impl Into<String>, data: impl Serialize) -> Result<Self, serde_json::Error> {
        Ok(Self {
            job_type: job_type.into(),
            data: serde_json::to_value(data)?,
            metadata: HashMap::new(),
        })
    }

    /// Add metadata to the payload
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Deserialize the data into a specific type
    pub fn deserialize<T: for<'de> Deserialize<'de>>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_value(self.data.clone())
    }
}

/// Retry configuration for a job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_attempts: u32,
    /// Initial delay between retries
    pub initial_delay: Duration,
    /// Multiplier for exponential backoff
    pub backoff_multiplier: f64,
    /// Maximum delay between retries
    pub max_delay: Duration,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_secs(1),
            backoff_multiplier: 2.0,
            max_delay: Duration::from_secs(3600), // 1 hour
        }
    }
}

impl RetryConfig {
    /// Calculate delay for a given attempt number
    pub fn delay_for_attempt(&self, attempt: u32) -> Duration {
        if attempt == 0 {
            return self.initial_delay;
        }

        let delay_secs = self.initial_delay.as_secs_f64()
            * self.backoff_multiplier.powi(attempt as i32);
        let delay = Duration::from_secs_f64(delay_secs);

        std::cmp::min(delay, self.max_delay)
    }
}

/// The main Job structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    /// Unique job identifier
    pub id: JobId,
    /// Queue this job belongs to
    pub queue: QueueName,
    /// Current state
    pub state: JobState,
    /// Job priority
    pub priority: Priority,
    /// The actual job payload
    pub payload: JobPayload,
    /// Retry configuration
    pub retry_config: RetryConfig,
    /// Current attempt number (0-indexed)
    pub attempt: u32,
    /// Worker currently processing this job
    pub worker_id: Option<WorkerId>,
    /// When the job was created
    pub created_at: Timestamp,
    /// When the job was last updated
    pub updated_at: Timestamp,
    /// When the job should be executed (for scheduled jobs)
    pub scheduled_at: Option<Timestamp>,
    /// When the job started processing
    pub started_at: Option<Timestamp>,
    /// When the job finished (completed, failed, or cancelled)
    pub finished_at: Option<Timestamp>,
    /// Job timeout
    pub timeout: Option<Duration>,
    /// Error message if job failed
    pub error: Option<String>,
    /// Jobs that must complete before this one
    pub dependencies: Vec<JobId>,
    /// Tags for filtering and grouping
    pub tags: Vec<String>,
    /// Progress percentage (0-100)
    pub progress: Option<u8>,
    /// Result data (if completed)
    pub result: Option<serde_json::Value>,
}

impl Job {
    /// Create a new job builder
    pub fn builder(job_type: impl Into<String>) -> JobBuilder {
        JobBuilder::new(job_type)
    }

    /// Check if the job can be retried
    pub fn can_retry(&self) -> bool {
        self.attempt < self.retry_config.max_attempts
    }

    /// Get the delay before next retry
    pub fn next_retry_delay(&self) -> Duration {
        self.retry_config.delay_for_attempt(self.attempt)
    }

    /// Transition to a new state
    pub fn transition_to(&mut self, new_state: JobState) -> Result<(), crate::error::JobQueueError> {
        if !self.state.can_transition_to(&new_state) {
            return Err(crate::error::JobQueueError::InvalidStateTransition {
                from: self.state,
                to: new_state,
            });
        }

        self.state = new_state;
        self.updated_at = Utc::now();

        match new_state {
            JobState::Running => {
                self.started_at = Some(Utc::now());
            }
            JobState::Completed | JobState::Failed | JobState::Cancelled => {
                self.finished_at = Some(Utc::now());
            }
            JobState::Retrying => {
                self.attempt += 1;
            }
            _ => {}
        }

        Ok(())
    }

    /// Update progress
    pub fn set_progress(&mut self, progress: u8) {
        self.progress = Some(progress.min(100));
        self.updated_at = Utc::now();
    }

    /// Check if all dependencies are satisfied
    pub fn dependencies_satisfied(&self, completed_jobs: &[JobId]) -> bool {
        self.dependencies.iter().all(|dep| completed_jobs.contains(dep))
    }
}

/// Builder for creating jobs
#[derive(Debug)]
pub struct JobBuilder {
    job_type: String,
    data: Option<serde_json::Value>,
    queue: QueueName,
    priority: Priority,
    retry_config: RetryConfig,
    scheduled_at: Option<Timestamp>,
    timeout: Option<Duration>,
    dependencies: Vec<JobId>,
    tags: Vec<String>,
    metadata: HashMap<String, String>,
}

impl JobBuilder {
    /// Create a new job builder
    pub fn new(job_type: impl Into<String>) -> Self {
        Self {
            job_type: job_type.into(),
            data: None,
            queue: QueueName::default(),
            priority: Priority::default(),
            retry_config: RetryConfig::default(),
            scheduled_at: None,
            timeout: None,
            dependencies: Vec::new(),
            tags: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Set the job data
    pub fn data(mut self, data: impl Serialize) -> Result<Self, serde_json::Error> {
        self.data = Some(serde_json::to_value(data)?);
        Ok(self)
    }

    /// Set raw JSON data
    pub fn data_raw(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }

    /// Set the queue
    pub fn queue(mut self, queue: impl Into<QueueName>) -> Self {
        self.queue = queue.into();
        self
    }

    /// Set priority
    pub fn priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }

    /// Set high priority
    pub fn high_priority(self) -> Self {
        self.priority(Priority::High)
    }

    /// Set critical priority
    pub fn critical(self) -> Self {
        self.priority(Priority::Critical)
    }

    /// Set retry configuration
    pub fn retry(mut self, config: RetryConfig) -> Self {
        self.retry_config = config;
        self
    }

    /// Set max retry attempts
    pub fn max_retries(mut self, max: u32) -> Self {
        self.retry_config.max_attempts = max;
        self
    }

    /// Schedule for later execution
    pub fn schedule_at(mut self, at: DateTime<Utc>) -> Self {
        self.scheduled_at = Some(at);
        self
    }

    /// Schedule after a delay
    pub fn schedule_in(self, delay: Duration) -> Self {
        self.schedule_at(Utc::now() + chrono::Duration::from_std(delay).unwrap())
    }

    /// Set timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Add a dependency
    pub fn depends_on(mut self, job_id: impl Into<JobId>) -> Self {
        self.dependencies.push(job_id.into());
        self
    }

    /// Add multiple dependencies
    pub fn depends_on_all(mut self, job_ids: impl IntoIterator<Item = impl Into<JobId>>) -> Self {
        self.dependencies.extend(job_ids.into_iter().map(Into::into));
        self
    }

    /// Add a tag
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Add metadata
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Build the job
    pub fn build(self) -> Job {
        let now = Utc::now();
        let initial_state = if self.scheduled_at.is_some() {
            JobState::Scheduled
        } else if !self.dependencies.is_empty() {
            JobState::Blocked
        } else {
            JobState::Pending
        };

        Job {
            id: JobId::new(),
            queue: self.queue,
            state: initial_state,
            priority: self.priority,
            payload: JobPayload {
                job_type: self.job_type,
                data: self.data.unwrap_or(serde_json::Value::Null),
                metadata: self.metadata,
            },
            retry_config: self.retry_config,
            attempt: 0,
            worker_id: None,
            created_at: now,
            updated_at: now,
            scheduled_at: self.scheduled_at,
            started_at: None,
            finished_at: None,
            timeout: self.timeout,
            error: None,
            dependencies: self.dependencies,
            tags: self.tags,
            progress: None,
            result: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_builder() {
        let job = Job::builder("email")
            .data(&serde_json::json!({"to": "test@example.com"}))
            .unwrap()
            .queue("emails")
            .priority(Priority::High)
            .max_retries(5)
            .tag("notification")
            .build();

        assert_eq!(job.payload.job_type, "email");
        assert_eq!(job.queue.as_str(), "emails");
        assert_eq!(job.priority, Priority::High);
        assert_eq!(job.retry_config.max_attempts, 5);
        assert!(job.tags.contains(&"notification".to_string()));
    }

    #[test]
    fn test_state_transitions() {
        let mut job = Job::builder("test").build();

        assert!(job.state.can_transition_to(&JobState::Running));
        assert!(job.transition_to(JobState::Running).is_ok());

        assert!(job.state.can_transition_to(&JobState::Completed));
        assert!(job.transition_to(JobState::Completed).is_ok());

        // Terminal state - no transitions allowed
        assert!(!job.state.can_transition_to(&JobState::Running));
    }

    #[test]
    fn test_retry_backoff() {
        let config = RetryConfig {
            max_attempts: 5,
            initial_delay: Duration::from_secs(1),
            backoff_multiplier: 2.0,
            max_delay: Duration::from_secs(60),
        };

        assert_eq!(config.delay_for_attempt(0), Duration::from_secs(1));
        assert_eq!(config.delay_for_attempt(1), Duration::from_secs(2));
        assert_eq!(config.delay_for_attempt(2), Duration::from_secs(4));
        assert_eq!(config.delay_for_attempt(3), Duration::from_secs(8));
    }
}
