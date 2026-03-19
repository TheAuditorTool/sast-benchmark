//! Metrics collection for monitoring JobQueue performance
//!
//! This module provides metrics collection and reporting.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

use crate::job::JobState;
use crate::types::QueueName;

/// Main metrics collector
#[derive(Debug)]
pub struct Metrics {
    /// Jobs processed counter
    jobs_processed: AtomicU64,
    /// Jobs failed counter
    jobs_failed: AtomicU64,
    /// Jobs retried counter
    jobs_retried: AtomicU64,
    /// Jobs currently in progress
    jobs_in_progress: AtomicU64,
    /// Total processing time (nanoseconds)
    total_processing_time_ns: AtomicU64,
    /// Per-queue metrics
    queue_metrics: RwLock<HashMap<QueueName, QueueMetrics>>,
    /// Per-job-type metrics
    job_type_metrics: RwLock<HashMap<String, JobTypeMetrics>>,
    /// Histogram buckets for latency
    latency_histogram: RwLock<LatencyHistogram>,
    /// Start time
    start_time: Instant,
}

impl Metrics {
    /// Create a new metrics collector
    pub fn new() -> Self {
        Self {
            jobs_processed: AtomicU64::new(0),
            jobs_failed: AtomicU64::new(0),
            jobs_retried: AtomicU64::new(0),
            jobs_in_progress: AtomicU64::new(0),
            total_processing_time_ns: AtomicU64::new(0),
            queue_metrics: RwLock::new(HashMap::new()),
            job_type_metrics: RwLock::new(HashMap::new()),
            latency_histogram: RwLock::new(LatencyHistogram::new()),
            start_time: Instant::now(),
        }
    }

    /// Record a job starting
    pub fn record_job_started(&self, queue: &QueueName, job_type: &str) {
        self.jobs_in_progress.fetch_add(1, Ordering::Relaxed);

        // Update queue metrics
        let mut queue_metrics = self.queue_metrics.write().unwrap();
        queue_metrics
            .entry(queue.clone())
            .or_insert_with(QueueMetrics::new)
            .jobs_started += 1;

        // Update job type metrics
        let mut type_metrics = self.job_type_metrics.write().unwrap();
        type_metrics
            .entry(job_type.to_string())
            .or_insert_with(JobTypeMetrics::new)
            .started += 1;
    }

    /// Record a job completed
    pub fn record_job_completed(&self, queue: &QueueName, job_type: &str, duration: Duration) {
        self.jobs_processed.fetch_add(1, Ordering::Relaxed);
        self.jobs_in_progress.fetch_sub(1, Ordering::Relaxed);
        self.total_processing_time_ns
            .fetch_add(duration.as_nanos() as u64, Ordering::Relaxed);

        // Update histogram
        {
            let mut histogram = self.latency_histogram.write().unwrap();
            histogram.record(duration);
        }

        // Update queue metrics
        {
            let mut queue_metrics = self.queue_metrics.write().unwrap();
            if let Some(qm) = queue_metrics.get_mut(queue) {
                qm.jobs_completed += 1;
                qm.total_duration_ms += duration.as_millis() as u64;
            }
        }

        // Update job type metrics
        {
            let mut type_metrics = self.job_type_metrics.write().unwrap();
            if let Some(tm) = type_metrics.get_mut(job_type) {
                tm.completed += 1;
                tm.total_duration_ms += duration.as_millis() as u64;
            }
        }
    }

    /// Record a job failed
    pub fn record_job_failed(&self, queue: &QueueName, job_type: &str, will_retry: bool) {
        self.jobs_failed.fetch_add(1, Ordering::Relaxed);
        self.jobs_in_progress.fetch_sub(1, Ordering::Relaxed);

        if will_retry {
            self.jobs_retried.fetch_add(1, Ordering::Relaxed);
        }

        // Update queue metrics
        {
            let mut queue_metrics = self.queue_metrics.write().unwrap();
            if let Some(qm) = queue_metrics.get_mut(queue) {
                qm.jobs_failed += 1;
            }
        }

        // Update job type metrics
        {
            let mut type_metrics = self.job_type_metrics.write().unwrap();
            if let Some(tm) = type_metrics.get_mut(job_type) {
                tm.failed += 1;
            }
        }
    }

    /// Get total jobs processed
    pub fn jobs_processed(&self) -> u64 {
        self.jobs_processed.load(Ordering::Relaxed)
    }

    /// Get total jobs failed
    pub fn jobs_failed(&self) -> u64 {
        self.jobs_failed.load(Ordering::Relaxed)
    }

    /// Get jobs currently in progress
    pub fn jobs_in_progress(&self) -> u64 {
        self.jobs_in_progress.load(Ordering::Relaxed)
    }

    /// Get average processing time
    pub fn average_processing_time(&self) -> Duration {
        let total_ns = self.total_processing_time_ns.load(Ordering::Relaxed);
        let count = self.jobs_processed.load(Ordering::Relaxed);
        if count == 0 {
            Duration::ZERO
        } else {
            Duration::from_nanos(total_ns / count)
        }
    }

    /// Get jobs per second rate
    pub fn jobs_per_second(&self) -> f64 {
        let elapsed = self.start_time.elapsed().as_secs_f64();
        if elapsed == 0.0 {
            0.0
        } else {
            self.jobs_processed.load(Ordering::Relaxed) as f64 / elapsed
        }
    }

    /// Get error rate (0.0 - 1.0)
    pub fn error_rate(&self) -> f64 {
        let total = self.jobs_processed.load(Ordering::Relaxed)
            + self.jobs_failed.load(Ordering::Relaxed);
        if total == 0 {
            0.0
        } else {
            self.jobs_failed.load(Ordering::Relaxed) as f64 / total as f64
        }
    }

    /// Get uptime
    pub fn uptime(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Get latency percentiles
    pub fn latency_percentiles(&self) -> LatencyPercentiles {
        let histogram = self.latency_histogram.read().unwrap();
        histogram.percentiles()
    }

    /// Get snapshot of all metrics
    pub fn snapshot(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            jobs_processed: self.jobs_processed(),
            jobs_failed: self.jobs_failed(),
            jobs_retried: self.jobs_retried.load(Ordering::Relaxed),
            jobs_in_progress: self.jobs_in_progress(),
            average_processing_time_ms: self.average_processing_time().as_millis() as u64,
            jobs_per_second: self.jobs_per_second(),
            error_rate: self.error_rate(),
            uptime_seconds: self.uptime().as_secs(),
            latency: self.latency_percentiles(),
            queues: self.queue_metrics.read().unwrap().clone(),
            job_types: self.job_type_metrics.read().unwrap().clone(),
        }
    }

    /// Export metrics in Prometheus format
    pub fn to_prometheus(&self) -> String {
        let snapshot = self.snapshot();
        let mut output = String::new();

        output.push_str("# HELP jobqueue_jobs_processed_total Total number of jobs processed\n");
        output.push_str("# TYPE jobqueue_jobs_processed_total counter\n");
        output.push_str(&format!("jobqueue_jobs_processed_total {}\n", snapshot.jobs_processed));

        output.push_str("# HELP jobqueue_jobs_failed_total Total number of jobs failed\n");
        output.push_str("# TYPE jobqueue_jobs_failed_total counter\n");
        output.push_str(&format!("jobqueue_jobs_failed_total {}\n", snapshot.jobs_failed));

        output.push_str("# HELP jobqueue_jobs_in_progress Current jobs being processed\n");
        output.push_str("# TYPE jobqueue_jobs_in_progress gauge\n");
        output.push_str(&format!("jobqueue_jobs_in_progress {}\n", snapshot.jobs_in_progress));

        output.push_str("# HELP jobqueue_processing_time_seconds Job processing time\n");
        output.push_str("# TYPE jobqueue_processing_time_seconds histogram\n");
        output.push_str(&format!(
            "jobqueue_processing_time_seconds{{quantile=\"0.5\"}} {:.3}\n",
            snapshot.latency.p50.as_secs_f64()
        ));
        output.push_str(&format!(
            "jobqueue_processing_time_seconds{{quantile=\"0.95\"}} {:.3}\n",
            snapshot.latency.p95.as_secs_f64()
        ));
        output.push_str(&format!(
            "jobqueue_processing_time_seconds{{quantile=\"0.99\"}} {:.3}\n",
            snapshot.latency.p99.as_secs_f64()
        ));

        // Per-queue metrics
        for (queue, metrics) in &snapshot.queues {
            output.push_str(&format!(
                "jobqueue_queue_jobs_completed{{queue=\"{}\"}} {}\n",
                queue, metrics.jobs_completed
            ));
            output.push_str(&format!(
                "jobqueue_queue_jobs_failed{{queue=\"{}\"}} {}\n",
                queue, metrics.jobs_failed
            ));
        }

        output
    }

    /// Reset all metrics
    pub fn reset(&self) {
        self.jobs_processed.store(0, Ordering::Relaxed);
        self.jobs_failed.store(0, Ordering::Relaxed);
        self.jobs_retried.store(0, Ordering::Relaxed);
        self.jobs_in_progress.store(0, Ordering::Relaxed);
        self.total_processing_time_ns.store(0, Ordering::Relaxed);
        self.queue_metrics.write().unwrap().clear();
        self.job_type_metrics.write().unwrap().clear();
        *self.latency_histogram.write().unwrap() = LatencyHistogram::new();
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Per-queue metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueueMetrics {
    pub jobs_started: u64,
    pub jobs_completed: u64,
    pub jobs_failed: u64,
    pub total_duration_ms: u64,
}

impl QueueMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn average_duration(&self) -> Duration {
        if self.jobs_completed == 0 {
            Duration::ZERO
        } else {
            Duration::from_millis(self.total_duration_ms / self.jobs_completed)
        }
    }
}

/// Per-job-type metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobTypeMetrics {
    pub started: u64,
    pub completed: u64,
    pub failed: u64,
    pub total_duration_ms: u64,
}

impl JobTypeMetrics {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Latency histogram for percentile calculations
#[derive(Debug)]
pub struct LatencyHistogram {
    /// All recorded durations (sorted on demand)
    samples: Vec<Duration>,
    /// Maximum samples to keep
    max_samples: usize,
}

impl LatencyHistogram {
    pub fn new() -> Self {
        Self::with_capacity(10000)
    }

    pub fn with_capacity(max_samples: usize) -> Self {
        Self {
            samples: Vec::with_capacity(max_samples.min(10000)),
            max_samples,
        }
    }

    pub fn record(&mut self, duration: Duration) {
        if self.samples.len() >= self.max_samples {
            // VULNERABILITY: No proper reservoir sampling
            // Just remove first element - biased towards recent values
            self.samples.remove(0);
        }
        self.samples.push(duration);
    }

    pub fn percentiles(&self) -> LatencyPercentiles {
        if self.samples.is_empty() {
            return LatencyPercentiles::default();
        }

        let mut sorted = self.samples.clone();
        sorted.sort();

        let percentile = |p: f64| -> Duration {
            let idx = ((sorted.len() as f64 * p) as usize).min(sorted.len() - 1);
            sorted[idx]
        };

        LatencyPercentiles {
            p50: percentile(0.50),
            p75: percentile(0.75),
            p90: percentile(0.90),
            p95: percentile(0.95),
            p99: percentile(0.99),
            max: *sorted.last().unwrap_or(&Duration::ZERO),
        }
    }
}

impl Default for LatencyHistogram {
    fn default() -> Self {
        Self::new()
    }
}

/// Latency percentiles
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LatencyPercentiles {
    #[serde(with = "duration_ms")]
    pub p50: Duration,
    #[serde(with = "duration_ms")]
    pub p75: Duration,
    #[serde(with = "duration_ms")]
    pub p90: Duration,
    #[serde(with = "duration_ms")]
    pub p95: Duration,
    #[serde(with = "duration_ms")]
    pub p99: Duration,
    #[serde(with = "duration_ms")]
    pub max: Duration,
}

mod duration_ms {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        duration.as_millis().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let ms = u64::deserialize(deserializer)?;
        Ok(Duration::from_millis(ms))
    }
}

/// Complete metrics snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSnapshot {
    pub jobs_processed: u64,
    pub jobs_failed: u64,
    pub jobs_retried: u64,
    pub jobs_in_progress: u64,
    pub average_processing_time_ms: u64,
    pub jobs_per_second: f64,
    pub error_rate: f64,
    pub uptime_seconds: u64,
    pub latency: LatencyPercentiles,
    pub queues: HashMap<QueueName, QueueMetrics>,
    pub job_types: HashMap<String, JobTypeMetrics>,
}

/// Thread-safe metrics wrapper
pub type SharedMetrics = Arc<Metrics>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_recording() {
        let metrics = Metrics::new();
        let queue = QueueName::new("test");

        metrics.record_job_started(&queue, "email");
        assert_eq!(metrics.jobs_in_progress(), 1);

        metrics.record_job_completed(&queue, "email", Duration::from_millis(100));
        assert_eq!(metrics.jobs_processed(), 1);
        assert_eq!(metrics.jobs_in_progress(), 0);
    }

    #[test]
    fn test_latency_histogram() {
        let mut histogram = LatencyHistogram::new();

        histogram.record(Duration::from_millis(10));
        histogram.record(Duration::from_millis(20));
        histogram.record(Duration::from_millis(30));
        histogram.record(Duration::from_millis(40));
        histogram.record(Duration::from_millis(50));

        let percentiles = histogram.percentiles();
        assert!(percentiles.p50 >= Duration::from_millis(20));
        assert!(percentiles.max == Duration::from_millis(50));
    }
}
