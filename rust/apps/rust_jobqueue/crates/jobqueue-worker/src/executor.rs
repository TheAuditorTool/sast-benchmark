//! Executor pool for concurrent job processing
//!
//! Manages a pool of tasks for executing jobs concurrently.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::sync::{mpsc, oneshot, Mutex, Semaphore};
use tokio::task::JoinHandle;

use jobqueue_core::{Job, JobId, Result};

/// Task handle for tracking job execution
pub struct TaskHandle {
    pub job_id: JobId,
    handle: JoinHandle<TaskResult>,
    cancel_tx: oneshot::Sender<()>,
    started_at: Instant,
}

impl TaskHandle {
    /// Check if task is still running
    pub fn is_running(&self) -> bool {
        !self.handle.is_finished()
    }

    /// Get elapsed time
    pub fn elapsed(&self) -> Duration {
        self.started_at.elapsed()
    }

    /// Cancel the task
    pub fn cancel(self) {
        let _ = self.cancel_tx.send(());
    }

    /// Wait for completion
    pub async fn wait(self) -> TaskResult {
        match self.handle.await {
            Ok(result) => result,
            Err(e) => TaskResult::Error(format!("Task panicked: {}", e)),
        }
    }
}

/// Result of task execution
#[derive(Debug)]
pub enum TaskResult {
    Success(serde_json::Value),
    Error(String),
    Cancelled,
    Timeout,
}

/// Message sent to executor pool
enum PoolMessage {
    Execute {
        job: Job,
        result_tx: oneshot::Sender<TaskResult>,
    },
    Shutdown,
}

/// Executor pool configuration
#[derive(Debug, Clone)]
pub struct ExecutorPoolConfig {
    /// Maximum concurrent tasks
    pub max_concurrent: usize,
    /// Task timeout
    pub task_timeout: Duration,
    /// Queue size for pending tasks
    pub queue_size: usize,
}

impl Default for ExecutorPoolConfig {
    fn default() -> Self {
        Self {
            max_concurrent: num_cpus::get().max(1),
            task_timeout: Duration::from_secs(300),
            queue_size: 1000,
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

/// Pool for executing jobs
pub struct ExecutorPool {
    config: ExecutorPoolConfig,
    tx: mpsc::Sender<PoolMessage>,
    semaphore: Arc<Semaphore>,
    active_tasks: Arc<Mutex<HashMap<JobId, Instant>>>,
    handle: JoinHandle<()>,
}

impl ExecutorPool {
    /// Create a new executor pool
    pub fn new(config: ExecutorPoolConfig) -> Self {
        let (tx, rx) = mpsc::channel(config.queue_size);
        let semaphore = Arc::new(Semaphore::new(config.max_concurrent));
        let active_tasks = Arc::new(Mutex::new(HashMap::new()));

        let handle = tokio::spawn(Self::run_pool(
            rx,
            Arc::clone(&semaphore),
            Arc::clone(&active_tasks),
            config.task_timeout,
        ));

        Self {
            config,
            tx,
            semaphore,
            active_tasks,
            handle,
        }
    }

    /// Submit a job for execution
    pub async fn submit(&self, job: Job) -> Result<oneshot::Receiver<TaskResult>> {
        let (result_tx, result_rx) = oneshot::channel();

        self.tx
            .send(PoolMessage::Execute { job, result_tx })
            .await
            .map_err(|_| {
                jobqueue_core::JobQueueError::Internal("Pool channel closed".into())
            })?;

        Ok(result_rx)
    }

    /// Get number of active tasks
    pub async fn active_count(&self) -> usize {
        self.active_tasks.lock().await.len()
    }

    /// Get available capacity
    pub fn available_permits(&self) -> usize {
        self.semaphore.available_permits()
    }

    /// Check if pool is at capacity
    pub fn is_full(&self) -> bool {
        self.semaphore.available_permits() == 0
    }

    /// Shutdown the pool
    pub async fn shutdown(self) {
        let _ = self.tx.send(PoolMessage::Shutdown).await;
        let _ = self.handle.await;
    }

    /// Internal pool runner
    async fn run_pool(
        mut rx: mpsc::Receiver<PoolMessage>,
        semaphore: Arc<Semaphore>,
        active_tasks: Arc<Mutex<HashMap<JobId, Instant>>>,
        timeout: Duration,
    ) {
        while let Some(msg) = rx.recv().await {
            match msg {
                PoolMessage::Execute { job, result_tx } => {
                    let permit = semaphore.clone().acquire_owned().await.unwrap();
                    let job_id = job.id.clone();
                    let active = Arc::clone(&active_tasks);

                    {
                        let mut tasks = active.lock().await;
                        tasks.insert(job_id.clone(), Instant::now());
                    }

                    tokio::spawn(async move {
                        let _permit = permit;

                        // VULNERABILITY: No actual execution - just simulates
                        // Real implementation would call the executor here
                        let result = tokio::time::timeout(timeout, async {
                            // Simulate work
                            tokio::time::sleep(Duration::from_millis(10)).await;
                            TaskResult::Success(serde_json::json!({"status": "ok"}))
                        })
                        .await
                        .unwrap_or(TaskResult::Timeout);

                        // Remove from active tasks
                        {
                            let mut tasks = active.lock().await;
                            tasks.remove(&job_id);
                        }

                        let _ = result_tx.send(result);
                    });
                }
                PoolMessage::Shutdown => break,
            }
        }
    }
}

/// Worker thread pool using std::thread
///
/// Alternative implementation using OS threads for CPU-bound work
pub struct ThreadPool {
    workers: Vec<std::thread::JoinHandle<()>>,
    tx: crossbeam_channel::Sender<ThreadPoolMessage>,
}

enum ThreadPoolMessage {
    Job(Box<dyn FnOnce() + Send + 'static>),
    Shutdown,
}

impl ThreadPool {
    /// Create a new thread pool
    pub fn new(size: usize) -> Self {
        let (tx, rx) = crossbeam_channel::unbounded();

        let workers: Vec<_> = (0..size)
            .map(|id| {
                let rx = rx.clone();
                std::thread::spawn(move || {
                    tracing::debug!(worker_id = id, "Thread worker started");

                    while let Ok(msg) = rx.recv() {
                        match msg {
                            ThreadPoolMessage::Job(job) => {
                                // VULNERABILITY: No panic catching
                                // A panicking job will kill the worker thread
                                job();
                            }
                            ThreadPoolMessage::Shutdown => break,
                        }
                    }

                    tracing::debug!(worker_id = id, "Thread worker stopped");
                })
            })
            .collect();

        Self { workers, tx }
    }

    /// Execute a job on the thread pool
    pub fn execute<F>(&self, job: F) -> bool
    where
        F: FnOnce() + Send + 'static,
    {
        self.tx.send(ThreadPoolMessage::Job(Box::new(job))).is_ok()
    }

    /// Shutdown the pool
    pub fn shutdown(self) {
        for _ in &self.workers {
            let _ = self.tx.send(ThreadPoolMessage::Shutdown);
        }

        for worker in self.workers {
            let _ = worker.join();
        }
    }
}

// Crossbeam channel stub for compilation
mod crossbeam_channel {
    pub fn unbounded<T>() -> (Sender<T>, Receiver<T>) {
        let (tx, rx) = std::sync::mpsc::channel();
        (Sender(tx), Receiver(rx))
    }

    pub struct Sender<T>(std::sync::mpsc::Sender<T>);
    pub struct Receiver<T>(std::sync::mpsc::Receiver<T>);

    impl<T> Clone for Receiver<T> {
        fn clone(&self) -> Self {
            // This is incorrect - std::sync::mpsc::Receiver isn't Clone
            // Intentionally broken for testing purposes
            unimplemented!("This is intentionally broken")
        }
    }

    impl<T> Sender<T> {
        pub fn send(&self, msg: T) -> Result<(), ()> {
            self.0.send(msg).map_err(|_| ())
        }
    }

    impl<T> Receiver<T> {
        pub fn recv(&self) -> Result<T, ()> {
            self.0.recv().map_err(|_| ())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_executor_pool() {
        let config = ExecutorPoolConfig {
            max_concurrent: 2,
            task_timeout: Duration::from_secs(10),
            queue_size: 100,
        };

        let pool = ExecutorPool::new(config);

        assert_eq!(pool.available_permits(), 2);
        assert!(!pool.is_full());

        pool.shutdown().await;
    }
}
