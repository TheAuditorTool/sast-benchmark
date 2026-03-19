//! Worker implementation
//!
//! The main worker that polls for jobs and processes them.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use chrono::Utc;
use tokio::sync::{broadcast, mpsc, Mutex, RwLock, Semaphore};
use tokio::time::{interval, timeout};

use jobqueue_core::{
    Job, JobEvent, JobExecutor, JobId, JobState, JobStore, Metrics,
    Priority, QueueName, Result, SharedMetrics, WorkerId,
};
use jobqueue_db::SqliteStore;

use crate::executor::ExecutorPool;
use crate::middleware::MiddlewareStack;

/// Worker configuration
#[derive(Debug, Clone)]
pub struct WorkerConfig {
    /// Worker identifier
    pub worker_id: WorkerId,
    /// Queues to process
    pub queues: Vec<QueueName>,
    /// Maximum concurrent jobs
    pub concurrency: usize,
    /// Poll interval
    pub poll_interval: Duration,
    /// Shutdown timeout
    pub shutdown_timeout: Duration,
    /// Heartbeat interval
    pub heartbeat_interval: Duration,
    /// Default job timeout
    pub job_timeout: Duration,
    /// Enable graceful shutdown
    pub graceful_shutdown: bool,
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self {
            worker_id: WorkerId::generate(),
            queues: vec![QueueName::default()],
            concurrency: num_cpus::get().max(1),
            poll_interval: Duration::from_millis(100),
            shutdown_timeout: Duration::from_secs(30),
            heartbeat_interval: Duration::from_secs(5),
            job_timeout: Duration::from_secs(300),
            graceful_shutdown: true,
        }
    }
}

fn num_cpus_get() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
}

mod num_cpus {
    pub fn get() -> usize {
        super::num_cpus_get()
    }
}

/// Worker state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkerState {
    /// Worker is idle
    Idle,
    /// Worker is processing jobs
    Running,
    /// Worker is shutting down
    ShuttingDown,
    /// Worker has stopped
    Stopped,
}

/// Worker handle for controlling the worker
pub struct WorkerHandle {
    shutdown_tx: broadcast::Sender<()>,
    state: Arc<RwLock<WorkerState>>,
}

impl WorkerHandle {
    /// Request graceful shutdown
    pub async fn shutdown(&self) {
        let _ = self.shutdown_tx.send(());
        let mut state = self.state.write().await;
        *state = WorkerState::ShuttingDown;
    }

    /// Get current state
    pub async fn state(&self) -> WorkerState {
        *self.state.read().await
    }

    /// Check if worker is running
    pub async fn is_running(&self) -> bool {
        matches!(self.state().await, WorkerState::Running)
    }
}

/// Main worker implementation
pub struct Worker<S: JobStore + Send + Sync + 'static> {
    config: WorkerConfig,
    store: Arc<S>,
    executors: Arc<RwLock<HashMap<String, Arc<dyn JobExecutor>>>>,
    middleware: Arc<MiddlewareStack>,
    metrics: SharedMetrics,
    state: Arc<RwLock<WorkerState>>,
    shutdown_tx: broadcast::Sender<()>,
    event_tx: Option<mpsc::Sender<JobEvent>>,
    semaphore: Arc<Semaphore>,
    active_jobs: Arc<Mutex<HashMap<JobId, tokio::task::JoinHandle<()>>>>,
}

impl<S: JobStore + Send + Sync + 'static> Worker<S> {
    /// Create a new worker
    pub fn new(store: S, config: WorkerConfig) -> Self {
        let (shutdown_tx, _) = broadcast::channel(1);
        let semaphore = Arc::new(Semaphore::new(config.concurrency));

        Self {
            config,
            store: Arc::new(store),
            executors: Arc::new(RwLock::new(HashMap::new())),
            middleware: Arc::new(MiddlewareStack::new()),
            metrics: Arc::new(Metrics::new()),
            state: Arc::new(RwLock::new(WorkerState::Idle)),
            shutdown_tx,
            event_tx: None,
            semaphore,
            active_jobs: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Register a job executor
    pub async fn register<E: JobExecutor + 'static>(&self, executor: E) {
        let job_type = executor.job_type().to_string();
        let mut executors = self.executors.write().await;
        executors.insert(job_type, Arc::new(executor));
    }

    /// Register a function as executor
    pub async fn register_fn<F, Fut>(
        &self,
        job_type: &str,
        func: F,
    ) where
        F: Fn(&Job) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<serde_json::Value>> + Send + 'static,
    {
        let executor = FnExecutor::new(job_type, func);
        self.register(executor).await;
    }

    /// Set event channel
    pub fn set_event_channel(&mut self, tx: mpsc::Sender<JobEvent>) {
        self.event_tx = Some(tx);
    }

    /// Get metrics
    pub fn metrics(&self) -> SharedMetrics {
        Arc::clone(&self.metrics)
    }

    /// Get worker handle
    pub fn handle(&self) -> WorkerHandle {
        WorkerHandle {
            shutdown_tx: self.shutdown_tx.clone(),
            state: Arc::clone(&self.state),
        }
    }

    /// Start the worker
    pub async fn start(&self) -> Result<()> {
        {
            let mut state = self.state.write().await;
            *state = WorkerState::Running;
        }

        tracing::info!(
            worker_id = %self.config.worker_id,
            queues = ?self.config.queues,
            concurrency = self.config.concurrency,
            "Worker starting"
        );

        let mut shutdown_rx = self.shutdown_tx.subscribe();
        let mut poll_interval = interval(self.config.poll_interval);
        let mut heartbeat_interval = interval(self.config.heartbeat_interval);

        loop {
            tokio::select! {
                _ = poll_interval.tick() => {
                    self.poll_and_process().await;
                }

                _ = heartbeat_interval.tick() => {
                    self.heartbeat().await;
                }

                _ = shutdown_rx.recv() => {
                    tracing::info!("Shutdown signal received");
                    break;
                }
            }
        }

        self.shutdown().await;
        Ok(())
    }

    /// Poll for jobs and process them
    async fn poll_and_process(&self) {
        // Check if we can accept more jobs
        let permit = match self.semaphore.clone().try_acquire_owned() {
            Ok(p) => p,
            Err(_) => return, // At capacity
        };

        // Try each queue in order
        for queue in &self.config.queues {
            match self.store.claim_next(queue, &self.config.worker_id).await {
                Ok(Some(job)) => {
                    self.process_job(job, permit).await;
                    return;
                }
                Ok(None) => continue,
                Err(e) => {
                    tracing::error!(error = %e, queue = %queue, "Failed to claim job");
                }
            }
        }

        // No jobs found, release permit
        drop(permit);
    }

    /// Process a single job
    async fn process_job(
        &self,
        job: Job,
        permit: tokio::sync::OwnedSemaphorePermit,
    ) {
        let job_id = job.id.clone();
        let job_type = job.payload.job_type.clone();
        let queue = job.queue.clone();
        let attempt = job.attempt;

        tracing::info!(
            job_id = %job_id,
            job_type = %job_type,
            queue = %queue,
            attempt = attempt,
            "Processing job"
        );

        // Emit started event
        self.emit_event(JobEvent::Started {
            job_id: job_id.clone(),
            worker_id: self.config.worker_id.clone(),
            attempt,
            timestamp: Utc::now(),
        }).await;

        // Record metrics
        self.metrics.record_job_started(&queue, &job_type);

        let start_time = std::time::Instant::now();
        let store = Arc::clone(&self.store);
        let executors = Arc::clone(&self.executors);
        let metrics = Arc::clone(&self.metrics);
        let event_tx = self.event_tx.clone();
        let job_timeout = job.timeout.unwrap_or(self.config.job_timeout);
        let worker_id = self.config.worker_id.clone();

        // Spawn task to process job
        let handle = tokio::spawn(async move {
            let _permit = permit; // Hold permit until done

            let result = timeout(job_timeout, async {
                let executors = executors.read().await;
                if let Some(executor) = executors.get(&job_type) {
                    executor.execute(&job).await
                } else {
                    Err(jobqueue_core::JobQueueError::Internal(
                        format!("No executor for job type: {}", job_type)
                    ))
                }
            }).await;

            let duration = start_time.elapsed();

            match result {
                Ok(Ok(result_data)) => {
                    // Job completed successfully
                    if let Err(e) = store.complete(&job_id, result_data.clone()).await {
                        tracing::error!(error = %e, job_id = %job_id, "Failed to mark job complete");
                    }

                    metrics.record_job_completed(&queue, &job_type, duration);

                    if let Some(tx) = &event_tx {
                        let _ = tx.send(JobEvent::Completed {
                            job_id,
                            duration_ms: duration.as_millis() as u64,
                            result: Some(result_data),
                            timestamp: Utc::now(),
                        }).await;
                    }
                }

                Ok(Err(e)) => {
                    // Job failed
                    let error_msg = e.to_string();
                    let will_retry = e.is_retryable() && job.can_retry();

                    if will_retry {
                        // Schedule retry
                        let delay = job.next_retry_delay();
                        if let Err(e) = store.update_state(&job_id, JobState::Retrying).await {
                            tracing::error!(error = %e, "Failed to set retry state");
                        }
                    } else {
                        if let Err(e) = store.fail(&job_id, &error_msg).await {
                            tracing::error!(error = %e, "Failed to mark job failed");
                        }
                    }

                    metrics.record_job_failed(&queue, &job_type, will_retry);

                    if let Some(tx) = &event_tx {
                        let _ = tx.send(JobEvent::Failed {
                            job_id,
                            error: error_msg,
                            attempt,
                            will_retry,
                            timestamp: Utc::now(),
                        }).await;
                    }
                }

                Err(_) => {
                    // Job timed out
                    let error_msg = format!("Job timed out after {:?}", job_timeout);
                    if let Err(e) = store.fail(&job_id, &error_msg).await {
                        tracing::error!(error = %e, "Failed to mark job timed out");
                    }

                    metrics.record_job_failed(&queue, &job_type, false);

                    if let Some(tx) = &event_tx {
                        let _ = tx.send(JobEvent::TimedOut {
                            job_id,
                            timeout_ms: job_timeout.as_millis() as u64,
                            timestamp: Utc::now(),
                        }).await;
                    }
                }
            }
        });

        // Track active job
        let mut active_jobs = self.active_jobs.lock().await;
        active_jobs.insert(job_id, handle);
    }

    /// Send heartbeat
    async fn heartbeat(&self) {
        let active_jobs = self.active_jobs.lock().await;
        let job_ids: Vec<JobId> = active_jobs.keys().cloned().collect();

        self.emit_event(JobEvent::Heartbeat {
            worker_id: self.config.worker_id.clone(),
            jobs_processing: job_ids,
            timestamp: Utc::now(),
        }).await;
    }

    /// Emit an event
    async fn emit_event(&self, event: JobEvent) {
        if let Some(tx) = &self.event_tx {
            let _ = tx.send(event).await;
        }
    }

    /// Graceful shutdown
    async fn shutdown(&self) {
        {
            let mut state = self.state.write().await;
            *state = WorkerState::ShuttingDown;
        }

        tracing::info!("Worker shutting down, waiting for active jobs...");

        if self.config.graceful_shutdown {
            // Wait for active jobs with timeout
            let shutdown_timeout = self.config.shutdown_timeout;
            let active_jobs = self.active_jobs.lock().await;

            let handles: Vec<_> = active_jobs.values().cloned().collect();
            drop(active_jobs);

            let wait_result = timeout(
                shutdown_timeout,
                futures::future::join_all(handles),
            ).await;

            match wait_result {
                Ok(_) => tracing::info!("All active jobs completed"),
                Err(_) => tracing::warn!("Shutdown timeout, some jobs may be interrupted"),
            }
        }

        {
            let mut state = self.state.write().await;
            *state = WorkerState::Stopped;
        }

        tracing::info!("Worker stopped");
    }
}

/// Function-based executor
struct FnExecutor<F, Fut>
where
    F: Fn(&Job) -> Fut + Send + Sync,
    Fut: std::future::Future<Output = Result<serde_json::Value>> + Send,
{
    job_type: String,
    func: F,
}

impl<F, Fut> FnExecutor<F, Fut>
where
    F: Fn(&Job) -> Fut + Send + Sync,
    Fut: std::future::Future<Output = Result<serde_json::Value>> + Send,
{
    fn new(job_type: impl Into<String>, func: F) -> Self {
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
    Fut: std::future::Future<Output = Result<serde_json::Value>> + Send,
{
    fn job_type(&self) -> &str {
        &self.job_type
    }

    async fn execute(&self, job: &Job) -> Result<serde_json::Value> {
        (self.func)(job).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_worker_config() {
        let config = WorkerConfig::default();
        assert!(!config.queues.is_empty());
        assert!(config.concurrency > 0);
    }
}
