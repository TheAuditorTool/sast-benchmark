//! Core traits for the JobQueue system
//!
//! These traits define the interfaces that different components must implement.

use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;

use crate::error::Result;
use crate::job::{Job, JobId, JobState};
use crate::types::{QueueName, WorkerId};

/// Trait for executing jobs
///
/// Implement this trait to handle specific job types.
///
/// # Example
///
/// ```rust,ignore
/// use jobqueue_core::{JobExecutor, Job, Result};
/// use async_trait::async_trait;
///
/// struct EmailSender;
///
/// #[async_trait]
/// impl JobExecutor for EmailSender {
///     fn job_type(&self) -> &str {
///         "send_email"
///     }
///
///     async fn execute(&self, job: &Job) -> Result<serde_json::Value> {
///         let email: EmailPayload = job.payload.deserialize()?;
///         // Send the email...
///         Ok(serde_json::json!({"sent": true}))
///     }
/// }
/// ```
#[async_trait]
pub trait JobExecutor: Send + Sync {
    /// The job type this executor handles
    fn job_type(&self) -> &str;

    /// Execute the job
    async fn execute(&self, job: &Job) -> Result<serde_json::Value>;

    /// Called before job execution (optional)
    async fn before_execute(&self, _job: &Job) -> Result<()> {
        Ok(())
    }

    /// Called after job execution (optional)
    async fn after_execute(&self, _job: &Job, _result: &Result<serde_json::Value>) -> Result<()> {
        Ok(())
    }

    /// Get executor metadata
    fn metadata(&self) -> ExecutorMetadata {
        ExecutorMetadata::default()
    }
}

/// Metadata about an executor
#[derive(Debug, Clone, Default)]
pub struct ExecutorMetadata {
    /// Executor name
    pub name: Option<String>,
    /// Executor version
    pub version: Option<String>,
    /// Maximum concurrent jobs this executor can handle
    pub max_concurrent: Option<usize>,
    /// Tags for this executor
    pub tags: Vec<String>,
}

/// Function-based executor wrapper
pub struct FnExecutor<F, Fut>
where
    F: Fn(&Job) -> Fut + Send + Sync,
    Fut: Future<Output = Result<serde_json::Value>> + Send,
{
    job_type: String,
    func: F,
}

impl<F, Fut> FnExecutor<F, Fut>
where
    F: Fn(&Job) -> Fut + Send + Sync,
    Fut: Future<Output = Result<serde_json::Value>> + Send,
{
    pub fn new(job_type: impl Into<String>, func: F) -> Self {
        Self {
            job_type: job_type.into(),
            func,
        }
    }
}

#[async_trait]
impl<F, Fut> JobExecutor for FnExecutor<F, Fut>
where
    F: Fn(&Job) -> Fut + Send + Sync,
    Fut: Future<Output = Result<serde_json::Value>> + Send,
{
    fn job_type(&self) -> &str {
        &self.job_type
    }

    async fn execute(&self, job: &Job) -> Result<serde_json::Value> {
        (self.func)(job).await
    }
}

/// Trait for storing and retrieving jobs
#[async_trait]
pub trait JobStore: Send + Sync {
    /// Save a job
    async fn save(&self, job: &Job) -> Result<()>;

    /// Get a job by ID
    async fn get(&self, id: &JobId) -> Result<Option<Job>>;

    /// Delete a job
    async fn delete(&self, id: &JobId) -> Result<bool>;

    /// Update job state
    async fn update_state(&self, id: &JobId, state: JobState) -> Result<()>;

    /// List jobs with optional filters
    async fn list(&self, filter: JobFilter) -> Result<Vec<Job>>;

    /// Count jobs matching filter
    async fn count(&self, filter: JobFilter) -> Result<u64>;

    /// Claim the next available job for processing
    async fn claim_next(&self, queue: &QueueName, worker: &WorkerId) -> Result<Option<Job>>;

    /// Release a claimed job back to the queue
    async fn release(&self, id: &JobId) -> Result<()>;

    /// Mark job as completed with result
    async fn complete(&self, id: &JobId, result: serde_json::Value) -> Result<()>;

    /// Mark job as failed with error
    async fn fail(&self, id: &JobId, error: &str) -> Result<()>;

    /// Get jobs ready for retry
    async fn get_retry_ready(&self) -> Result<Vec<Job>>;

    /// Get scheduled jobs that are due
    async fn get_scheduled_due(&self) -> Result<Vec<Job>>;

    /// Clean up old completed/failed jobs
    async fn cleanup(&self, older_than: chrono::Duration) -> Result<u64>;
}

/// Filter for listing jobs
#[derive(Debug, Clone, Default)]
pub struct JobFilter {
    /// Filter by queue
    pub queue: Option<QueueName>,
    /// Filter by state
    pub state: Option<JobState>,
    /// Filter by job type
    pub job_type: Option<String>,
    /// Filter by tags (any match)
    pub tags: Vec<String>,
    /// Filter by worker
    pub worker_id: Option<WorkerId>,
    /// Maximum number of results
    pub limit: Option<usize>,
    /// Offset for pagination
    pub offset: Option<usize>,
    /// Sort order
    pub order: SortOrder,
}

/// Sort order for job listings
#[derive(Debug, Clone, Copy, Default)]
pub enum SortOrder {
    /// Oldest first
    #[default]
    OldestFirst,
    /// Newest first
    NewestFirst,
    /// Highest priority first
    PriorityDesc,
    /// Lowest priority first
    PriorityAsc,
}

impl JobFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn queue(mut self, queue: impl Into<QueueName>) -> Self {
        self.queue = Some(queue.into());
        self
    }

    pub fn state(mut self, state: JobState) -> Self {
        self.state = Some(state);
        self
    }

    pub fn job_type(mut self, job_type: impl Into<String>) -> Self {
        self.job_type = Some(job_type.into());
        self
    }

    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    pub fn worker(mut self, worker: impl Into<WorkerId>) -> Self {
        self.worker_id = Some(worker.into());
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn order(mut self, order: SortOrder) -> Self {
        self.order = order;
        self
    }
}

/// Trait for queue backend operations
#[async_trait]
pub trait QueueBackend: Send + Sync {
    /// Push a job to the queue
    async fn push(&self, job: Job) -> Result<JobId>;

    /// Pop the next job from the queue
    async fn pop(&self, queue: &QueueName) -> Result<Option<Job>>;

    /// Peek at the next job without removing it
    async fn peek(&self, queue: &QueueName) -> Result<Option<Job>>;

    /// Get queue length
    async fn len(&self, queue: &QueueName) -> Result<usize>;

    /// Check if queue is empty
    async fn is_empty(&self, queue: &QueueName) -> Result<bool> {
        Ok(self.len(queue).await? == 0)
    }

    /// Get all queue names
    async fn queues(&self) -> Result<Vec<QueueName>>;

    /// Pause a queue
    async fn pause(&self, queue: &QueueName) -> Result<()>;

    /// Resume a paused queue
    async fn resume(&self, queue: &QueueName) -> Result<()>;

    /// Check if queue is paused
    async fn is_paused(&self, queue: &QueueName) -> Result<bool>;

    /// Clear all jobs from a queue
    async fn clear(&self, queue: &QueueName) -> Result<u64>;
}

/// Type alias for boxed executor
pub type BoxedExecutor = Box<dyn JobExecutor>;

/// Type alias for async job execution function
pub type AsyncJobFn = Box<
    dyn Fn(&Job) -> Pin<Box<dyn Future<Output = Result<serde_json::Value>> + Send>>
        + Send
        + Sync,
>;

/// Middleware trait for job processing pipeline
#[async_trait]
pub trait JobMiddleware: Send + Sync {
    /// Process before job execution
    async fn before(&self, job: &mut Job) -> Result<()>;

    /// Process after job execution
    async fn after(&self, job: &Job, result: &mut Result<serde_json::Value>) -> Result<()>;
}

/// Extension trait for JobStore with additional operations
#[async_trait]
pub trait JobStoreExt: JobStore {
    /// Batch save jobs
    async fn save_batch(&self, jobs: &[Job]) -> Result<()> {
        for job in jobs {
            self.save(job).await?;
        }
        Ok(())
    }

    /// Get jobs by IDs
    async fn get_batch(&self, ids: &[JobId]) -> Result<Vec<Job>> {
        let mut jobs = Vec::with_capacity(ids.len());
        for id in ids {
            if let Some(job) = self.get(id).await? {
                jobs.push(job);
            }
        }
        Ok(jobs)
    }

    /// Check if job exists
    async fn exists(&self, id: &JobId) -> Result<bool> {
        Ok(self.get(id).await?.is_some())
    }
}

// Blanket implementation
impl<T: JobStore> JobStoreExt for T {}
