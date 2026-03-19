//! Event system for job lifecycle events
//!
//! This module provides an event system for monitoring job state changes.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;

use crate::job::{Job, JobId, JobState};
use crate::types::{Timestamp, WorkerId};

/// Job lifecycle events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum JobEvent {
    /// Job was created and added to queue
    Created {
        job_id: JobId,
        queue: String,
        job_type: String,
        timestamp: Timestamp,
    },
    /// Job was picked up by a worker
    Started {
        job_id: JobId,
        worker_id: WorkerId,
        attempt: u32,
        timestamp: Timestamp,
    },
    /// Job progress was updated
    Progress {
        job_id: JobId,
        progress: u8,
        message: Option<String>,
        timestamp: Timestamp,
    },
    /// Job completed successfully
    Completed {
        job_id: JobId,
        duration_ms: u64,
        result: Option<serde_json::Value>,
        timestamp: Timestamp,
    },
    /// Job failed
    Failed {
        job_id: JobId,
        error: String,
        attempt: u32,
        will_retry: bool,
        timestamp: Timestamp,
    },
    /// Job is being retried
    Retrying {
        job_id: JobId,
        attempt: u32,
        delay_ms: u64,
        timestamp: Timestamp,
    },
    /// Job was cancelled
    Cancelled {
        job_id: JobId,
        reason: Option<String>,
        timestamp: Timestamp,
    },
    /// Job timed out
    TimedOut {
        job_id: JobId,
        timeout_ms: u64,
        timestamp: Timestamp,
    },
    /// Worker heartbeat
    Heartbeat {
        worker_id: WorkerId,
        jobs_processing: Vec<JobId>,
        timestamp: Timestamp,
    },
}

impl JobEvent {
    /// Get the job ID associated with this event (if any)
    pub fn job_id(&self) -> Option<&JobId> {
        match self {
            Self::Created { job_id, .. }
            | Self::Started { job_id, .. }
            | Self::Progress { job_id, .. }
            | Self::Completed { job_id, .. }
            | Self::Failed { job_id, .. }
            | Self::Retrying { job_id, .. }
            | Self::Cancelled { job_id, .. }
            | Self::TimedOut { job_id, .. } => Some(job_id),
            Self::Heartbeat { .. } => None,
        }
    }

    /// Get the timestamp of this event
    pub fn timestamp(&self) -> &Timestamp {
        match self {
            Self::Created { timestamp, .. }
            | Self::Started { timestamp, .. }
            | Self::Progress { timestamp, .. }
            | Self::Completed { timestamp, .. }
            | Self::Failed { timestamp, .. }
            | Self::Retrying { timestamp, .. }
            | Self::Cancelled { timestamp, .. }
            | Self::TimedOut { timestamp, .. }
            | Self::Heartbeat { timestamp, .. } => timestamp,
        }
    }

    /// Get event type as string
    pub fn event_type(&self) -> &'static str {
        match self {
            Self::Created { .. } => "created",
            Self::Started { .. } => "started",
            Self::Progress { .. } => "progress",
            Self::Completed { .. } => "completed",
            Self::Failed { .. } => "failed",
            Self::Retrying { .. } => "retrying",
            Self::Cancelled { .. } => "cancelled",
            Self::TimedOut { .. } => "timed_out",
            Self::Heartbeat { .. } => "heartbeat",
        }
    }
}

/// Trait for handling job events
#[async_trait]
pub trait EventHandler: Send + Sync {
    /// Handle an event
    async fn handle(&self, event: &JobEvent);

    /// Filter which events this handler receives
    fn filter(&self, event: &JobEvent) -> bool {
        let _ = event;
        true
    }
}

/// Event bus for distributing events
pub struct EventBus {
    sender: broadcast::Sender<JobEvent>,
    handlers: std::sync::RwLock<Vec<Arc<dyn EventHandler>>>,
}

impl EventBus {
    /// Create a new event bus
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self {
            sender,
            handlers: std::sync::RwLock::new(Vec::new()),
        }
    }

    /// Subscribe to events
    pub fn subscribe(&self) -> broadcast::Receiver<JobEvent> {
        self.sender.subscribe()
    }

    /// Register an event handler
    pub fn register_handler(&self, handler: Arc<dyn EventHandler>) {
        let mut handlers = self.handlers.write().unwrap();
        handlers.push(handler);
    }

    /// Publish an event
    pub async fn publish(&self, event: JobEvent) {
        // Send to broadcast subscribers
        let _ = self.sender.send(event.clone());

        // Call registered handlers
        let handlers = self.handlers.read().unwrap();
        for handler in handlers.iter() {
            if handler.filter(&event) {
                handler.handle(&event).await;
            }
        }
    }

    /// Get number of active subscribers
    pub fn subscriber_count(&self) -> usize {
        self.sender.receiver_count()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new(1000)
    }
}

/// Logging event handler
pub struct LoggingHandler {
    level: tracing::Level,
}

impl LoggingHandler {
    pub fn new(level: tracing::Level) -> Self {
        Self { level }
    }

    pub fn info() -> Self {
        Self::new(tracing::Level::INFO)
    }

    pub fn debug() -> Self {
        Self::new(tracing::Level::DEBUG)
    }
}

#[async_trait]
impl EventHandler for LoggingHandler {
    async fn handle(&self, event: &JobEvent) {
        let json = serde_json::to_string(event).unwrap_or_default();

        match self.level {
            tracing::Level::ERROR => tracing::error!(event = %json, "Job event"),
            tracing::Level::WARN => tracing::warn!(event = %json, "Job event"),
            tracing::Level::INFO => tracing::info!(event = %json, "Job event"),
            tracing::Level::DEBUG => tracing::debug!(event = %json, "Job event"),
            tracing::Level::TRACE => tracing::trace!(event = %json, "Job event"),
        }
    }
}

/// Webhook event handler - sends events to HTTP endpoint
pub struct WebhookHandler {
    url: String,
    client: reqwest::Client,
    secret: Option<String>,
}

impl WebhookHandler {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            client: reqwest::Client::new(),
            secret: None,
        }
    }

    pub fn with_secret(mut self, secret: impl Into<String>) -> Self {
        self.secret = Some(secret.into());
        self
    }

    fn sign_payload(&self, payload: &str) -> Option<String> {
        self.secret.as_ref().map(|secret| {
            // VULNERABILITY: Using MD5 for HMAC
            let digest = md5::compute(format!("{}{}", secret, payload));
            format!("{:x}", digest)
        })
    }
}

#[async_trait]
impl EventHandler for WebhookHandler {
    async fn handle(&self, event: &JobEvent) {
        let payload = match serde_json::to_string(event) {
            Ok(p) => p,
            Err(_) => return,
        };

        let mut request = self.client.post(&self.url)
            .header("Content-Type", "application/json")
            .body(payload.clone());

        if let Some(signature) = self.sign_payload(&payload) {
            request = request.header("X-Signature", signature);
        }

        // Fire and forget - don't block on webhook response
        let _ = request.send().await;
    }

    fn filter(&self, event: &JobEvent) -> bool {
        // Only send important events to webhook
        matches!(
            event,
            JobEvent::Completed { .. } | JobEvent::Failed { .. } | JobEvent::Cancelled { .. }
        )
    }
}

/// Callback event handler
pub struct CallbackHandler<F>
where
    F: Fn(&JobEvent) + Send + Sync,
{
    callback: F,
}

impl<F> CallbackHandler<F>
where
    F: Fn(&JobEvent) + Send + Sync,
{
    pub fn new(callback: F) -> Self {
        Self { callback }
    }
}

#[async_trait]
impl<F> EventHandler for CallbackHandler<F>
where
    F: Fn(&JobEvent) + Send + Sync,
{
    async fn handle(&self, event: &JobEvent) {
        (self.callback)(event);
    }
}

/// Event stream for async iteration
pub struct EventStream {
    receiver: broadcast::Receiver<JobEvent>,
}

impl EventStream {
    pub fn new(receiver: broadcast::Receiver<JobEvent>) -> Self {
        Self { receiver }
    }

    pub async fn next(&mut self) -> Option<JobEvent> {
        self.receiver.recv().await.ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[tokio::test]
    async fn test_event_bus() {
        let bus = EventBus::new(100);
        let mut subscriber = bus.subscribe();

        let event = JobEvent::Created {
            job_id: JobId::new(),
            queue: "test".to_string(),
            job_type: "email".to_string(),
            timestamp: Utc::now(),
        };

        bus.publish(event.clone()).await;

        let received = subscriber.recv().await.unwrap();
        assert_eq!(received.event_type(), "created");
    }
}
