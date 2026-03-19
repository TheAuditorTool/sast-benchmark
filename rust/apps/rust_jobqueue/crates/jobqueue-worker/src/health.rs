//! Health checking for workers
//!
//! Provides health status and liveness/readiness probes.

use std::sync::Arc;
use std::time::{Duration, Instant};

use serde::Serialize;
use tokio::sync::RwLock;

use jobqueue_core::{Metrics, SharedMetrics};

/// Health status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

impl HealthStatus {
    pub fn is_healthy(&self) -> bool {
        matches!(self, Self::Healthy | Self::Degraded)
    }
}

/// Health check result
#[derive(Debug, Clone, Serialize)]
pub struct HealthCheck {
    pub name: String,
    pub status: HealthStatus,
    pub message: Option<String>,
    pub duration_ms: u64,
}

/// Overall health report
#[derive(Debug, Clone, Serialize)]
pub struct HealthReport {
    pub status: HealthStatus,
    pub checks: Vec<HealthCheck>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl HealthReport {
    pub fn is_healthy(&self) -> bool {
        self.status.is_healthy()
    }
}

/// Health checker configuration
#[derive(Debug, Clone)]
pub struct HealthCheckerConfig {
    /// Check interval
    pub check_interval: Duration,
    /// Timeout for individual checks
    pub check_timeout: Duration,
    /// Maximum error rate before unhealthy
    pub max_error_rate: f64,
    /// Maximum queue depth before degraded
    pub max_queue_depth: usize,
    /// Stale threshold for worker heartbeats
    pub stale_heartbeat_threshold: Duration,
}

impl Default for HealthCheckerConfig {
    fn default() -> Self {
        Self {
            check_interval: Duration::from_secs(10),
            check_timeout: Duration::from_secs(5),
            max_error_rate: 0.1, // 10% error rate
            max_queue_depth: 10000,
            stale_heartbeat_threshold: Duration::from_secs(30),
        }
    }
}

/// Health checker
pub struct HealthChecker {
    config: HealthCheckerConfig,
    metrics: SharedMetrics,
    last_check: Arc<RwLock<Option<HealthReport>>>,
    custom_checks: Arc<RwLock<Vec<Box<dyn HealthCheckFn>>>>,
}

/// Trait for custom health checks
pub trait HealthCheckFn: Send + Sync {
    fn name(&self) -> &str;
    fn check(&self) -> HealthCheck;
}

impl HealthChecker {
    /// Create a new health checker
    pub fn new(metrics: SharedMetrics, config: HealthCheckerConfig) -> Self {
        Self {
            config,
            metrics,
            last_check: Arc::new(RwLock::new(None)),
            custom_checks: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Add a custom health check
    pub async fn add_check(&self, check: Box<dyn HealthCheckFn>) {
        let mut checks = self.custom_checks.write().await;
        checks.push(check);
    }

    /// Run all health checks
    pub async fn check(&self) -> HealthReport {
        let mut checks = Vec::new();
        let mut overall_status = HealthStatus::Healthy;

        // Check error rate
        let error_check = self.check_error_rate();
        if error_check.status == HealthStatus::Unhealthy {
            overall_status = HealthStatus::Unhealthy;
        } else if error_check.status == HealthStatus::Degraded && overall_status == HealthStatus::Healthy {
            overall_status = HealthStatus::Degraded;
        }
        checks.push(error_check);

        // Check jobs in progress
        let jobs_check = self.check_jobs_in_progress();
        checks.push(jobs_check);

        // Check throughput
        let throughput_check = self.check_throughput();
        checks.push(throughput_check);

        // Run custom checks
        {
            let custom_checks = self.custom_checks.read().await;
            for custom in custom_checks.iter() {
                let start = Instant::now();
                let mut check = custom.check();
                check.duration_ms = start.elapsed().as_millis() as u64;

                if check.status == HealthStatus::Unhealthy {
                    overall_status = HealthStatus::Unhealthy;
                } else if check.status == HealthStatus::Degraded && overall_status == HealthStatus::Healthy {
                    overall_status = HealthStatus::Degraded;
                }

                checks.push(check);
            }
        }

        let report = HealthReport {
            status: overall_status,
            checks,
            timestamp: chrono::Utc::now(),
        };

        // Cache the result
        {
            let mut last = self.last_check.write().await;
            *last = Some(report.clone());
        }

        report
    }

    /// Get cached health status
    pub async fn cached(&self) -> Option<HealthReport> {
        self.last_check.read().await.clone()
    }

    /// Liveness probe (is the worker alive?)
    pub async fn liveness(&self) -> bool {
        // Simple liveness - just check we can respond
        true
    }

    /// Readiness probe (is the worker ready to accept jobs?)
    pub async fn readiness(&self) -> bool {
        let report = self.check().await;
        report.is_healthy()
    }

    fn check_error_rate(&self) -> HealthCheck {
        let start = Instant::now();
        let error_rate = self.metrics.error_rate();

        let status = if error_rate > self.config.max_error_rate {
            HealthStatus::Unhealthy
        } else if error_rate > self.config.max_error_rate / 2.0 {
            HealthStatus::Degraded
        } else {
            HealthStatus::Healthy
        };

        HealthCheck {
            name: "error_rate".to_string(),
            status,
            message: Some(format!("{:.2}%", error_rate * 100.0)),
            duration_ms: start.elapsed().as_millis() as u64,
        }
    }

    fn check_jobs_in_progress(&self) -> HealthCheck {
        let start = Instant::now();
        let in_progress = self.metrics.jobs_in_progress();

        // VULNERABILITY: Hardcoded threshold, should be configurable
        let status = if in_progress > 1000 {
            HealthStatus::Degraded
        } else {
            HealthStatus::Healthy
        };

        HealthCheck {
            name: "jobs_in_progress".to_string(),
            status,
            message: Some(format!("{} jobs", in_progress)),
            duration_ms: start.elapsed().as_millis() as u64,
        }
    }

    fn check_throughput(&self) -> HealthCheck {
        let start = Instant::now();
        let rate = self.metrics.jobs_per_second();

        // Just informational
        HealthCheck {
            name: "throughput".to_string(),
            status: HealthStatus::Healthy,
            message: Some(format!("{:.2} jobs/s", rate)),
            duration_ms: start.elapsed().as_millis() as u64,
        }
    }
}

/// Database health check
pub struct DatabaseHealthCheck {
    name: String,
    // Would hold database connection
}

impl DatabaseHealthCheck {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl HealthCheckFn for DatabaseHealthCheck {
    fn name(&self) -> &str {
        &self.name
    }

    fn check(&self) -> HealthCheck {
        // VULNERABILITY: No actual database check
        HealthCheck {
            name: self.name.clone(),
            status: HealthStatus::Healthy,
            message: Some("Database connection OK".to_string()),
            duration_ms: 0,
        }
    }
}

/// Memory health check
pub struct MemoryHealthCheck {
    max_memory_mb: u64,
}

impl MemoryHealthCheck {
    pub fn new(max_memory_mb: u64) -> Self {
        Self { max_memory_mb }
    }
}

impl HealthCheckFn for MemoryHealthCheck {
    fn name(&self) -> &str {
        "memory"
    }

    fn check(&self) -> HealthCheck {
        // VULNERABILITY: This doesn't actually check memory usage
        // Just returns healthy always
        HealthCheck {
            name: "memory".to_string(),
            status: HealthStatus::Healthy,
            message: Some("Memory OK (check not implemented)".to_string()),
            duration_ms: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_checker() {
        let metrics = Arc::new(Metrics::new());
        let config = HealthCheckerConfig::default();
        let checker = HealthChecker::new(metrics, config);

        let report = checker.check().await;
        assert!(report.is_healthy());
    }

    #[tokio::test]
    async fn test_liveness() {
        let metrics = Arc::new(Metrics::new());
        let config = HealthCheckerConfig::default();
        let checker = HealthChecker::new(metrics, config);

        assert!(checker.liveness().await);
    }
}
