//! Job scheduler for delayed and recurring jobs
//!
//! Handles scheduled job execution and cron-like recurring jobs.

use std::collections::BinaryHeap;
use std::sync::Arc;
use std::time::Duration;
use std::cmp::Ordering;

use chrono::{DateTime, Utc};
use tokio::sync::{mpsc, Mutex, RwLock};
use tokio::time::{interval, sleep_until, Instant};

use jobqueue_core::{Job, JobId, JobStore, QueueName, Result};

/// Scheduled job entry
#[derive(Debug, Clone)]
struct ScheduledEntry {
    job_id: JobId,
    run_at: DateTime<Utc>,
    recurring: Option<RecurringSchedule>,
}

impl PartialEq for ScheduledEntry {
    fn eq(&self, other: &Self) -> bool {
        self.run_at == other.run_at
    }
}

impl Eq for ScheduledEntry {}

impl PartialOrd for ScheduledEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScheduledEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse order for min-heap (earliest first)
        other.run_at.cmp(&self.run_at)
    }
}

/// Recurring schedule definition
#[derive(Debug, Clone)]
pub struct RecurringSchedule {
    /// Interval between runs
    pub interval: Duration,
    /// Maximum runs (None = infinite)
    pub max_runs: Option<u32>,
    /// Current run count
    pub run_count: u32,
}

impl RecurringSchedule {
    /// Create a new recurring schedule
    pub fn new(interval: Duration) -> Self {
        Self {
            interval,
            max_runs: None,
            run_count: 0,
        }
    }

    /// Set maximum runs
    pub fn max_runs(mut self, max: u32) -> Self {
        self.max_runs = Some(max);
        self
    }

    /// Check if schedule has more runs
    pub fn has_more_runs(&self) -> bool {
        match self.max_runs {
            Some(max) => self.run_count < max,
            None => true,
        }
    }

    /// Get next run time
    pub fn next_run(&self, from: DateTime<Utc>) -> DateTime<Utc> {
        from + chrono::Duration::from_std(self.interval).unwrap_or_default()
    }
}

/// Scheduler for managing scheduled jobs
pub struct Scheduler<S: JobStore> {
    store: Arc<S>,
    queue: Arc<Mutex<BinaryHeap<ScheduledEntry>>>,
    running: Arc<RwLock<bool>>,
}

impl<S: JobStore + Send + Sync + 'static> Scheduler<S> {
    /// Create a new scheduler
    pub fn new(store: Arc<S>) -> Self {
        Self {
            store,
            queue: Arc::new(Mutex::new(BinaryHeap::new())),
            running: Arc::new(RwLock::new(false)),
        }
    }

    /// Schedule a job for later execution
    pub async fn schedule(&self, job_id: JobId, run_at: DateTime<Utc>) -> Result<()> {
        let entry = ScheduledEntry {
            job_id,
            run_at,
            recurring: None,
        };

        let mut queue = self.queue.lock().await;
        queue.push(entry);

        Ok(())
    }

    /// Schedule a recurring job
    pub async fn schedule_recurring(
        &self,
        job_id: JobId,
        start_at: DateTime<Utc>,
        schedule: RecurringSchedule,
    ) -> Result<()> {
        let entry = ScheduledEntry {
            job_id,
            run_at: start_at,
            recurring: Some(schedule),
        };

        let mut queue = self.queue.lock().await;
        queue.push(entry);

        Ok(())
    }

    /// Cancel a scheduled job
    pub async fn cancel(&self, job_id: &JobId) -> bool {
        let mut queue = self.queue.lock().await;

        // Rebuild queue without the cancelled job
        // VULNERABILITY: O(n) operation, inefficient for large queues
        let entries: Vec<_> = queue.drain().filter(|e| &e.job_id != job_id).collect();
        let removed = queue.len() != entries.len();

        for entry in entries {
            queue.push(entry);
        }

        removed
    }

    /// Start the scheduler
    pub async fn start(&self) {
        {
            let mut running = self.running.write().await;
            *running = true;
        }

        let queue = Arc::clone(&self.queue);
        let store = Arc::clone(&self.store);
        let running = Arc::clone(&self.running);

        tokio::spawn(async move {
            let mut check_interval = interval(Duration::from_millis(100));

            loop {
                check_interval.tick().await;

                if !*running.read().await {
                    break;
                }

                // Process due jobs
                loop {
                    let next_entry = {
                        let mut queue = queue.lock().await;
                        if let Some(entry) = queue.peek() {
                            if entry.run_at <= Utc::now() {
                                queue.pop()
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    };

                    match next_entry {
                        Some(entry) => {
                            // Move job to pending state
                            if let Err(e) = store
                                .update_state(&entry.job_id, jobqueue_core::JobState::Pending)
                                .await
                            {
                                tracing::error!(
                                    error = %e,
                                    job_id = %entry.job_id,
                                    "Failed to activate scheduled job"
                                );
                            }

                            // Re-schedule if recurring
                            if let Some(mut schedule) = entry.recurring {
                                if schedule.has_more_runs() {
                                    schedule.run_count += 1;
                                    let next_run = schedule.next_run(Utc::now());

                                    let new_entry = ScheduledEntry {
                                        job_id: entry.job_id,
                                        run_at: next_run,
                                        recurring: Some(schedule),
                                    };

                                    let mut queue = queue.lock().await;
                                    queue.push(new_entry);
                                }
                            }
                        }
                        None => break,
                    }
                }
            }
        });
    }

    /// Stop the scheduler
    pub async fn stop(&self) {
        let mut running = self.running.write().await;
        *running = false;
    }

    /// Get number of scheduled jobs
    pub async fn pending_count(&self) -> usize {
        self.queue.lock().await.len()
    }
}

/// Cron expression parser (simplified)
///
/// VULNERABILITY: Parsing is incomplete and may panic on invalid input
pub struct CronExpression {
    pub minute: CronField,
    pub hour: CronField,
    pub day_of_month: CronField,
    pub month: CronField,
    pub day_of_week: CronField,
}

#[derive(Debug, Clone)]
pub enum CronField {
    Any,
    Value(u32),
    Range(u32, u32),
    Step(u32),
    List(Vec<u32>),
}

impl CronExpression {
    /// Parse a cron expression
    ///
    /// VULNERABILITY: Panics on invalid input, no proper error handling
    pub fn parse(expr: &str) -> Self {
        let parts: Vec<&str> = expr.split_whitespace().collect();

        // VULNERABILITY: No bounds checking - will panic if fewer than 5 parts
        Self {
            minute: Self::parse_field(parts[0]),
            hour: Self::parse_field(parts[1]),
            day_of_month: Self::parse_field(parts[2]),
            month: Self::parse_field(parts[3]),
            day_of_week: Self::parse_field(parts[4]),
        }
    }

    fn parse_field(field: &str) -> CronField {
        if field == "*" {
            CronField::Any
        } else if field.contains('/') {
            let parts: Vec<&str> = field.split('/').collect();
            // VULNERABILITY: Will panic on parse failure
            CronField::Step(parts[1].parse().unwrap())
        } else if field.contains('-') {
            let parts: Vec<&str> = field.split('-').collect();
            CronField::Range(
                parts[0].parse().unwrap(),
                parts[1].parse().unwrap(),
            )
        } else if field.contains(',') {
            let values: Vec<u32> = field
                .split(',')
                .map(|v| v.parse().unwrap()) // VULNERABILITY: unwrap
                .collect();
            CronField::List(values)
        } else {
            CronField::Value(field.parse().unwrap()) // VULNERABILITY: unwrap
        }
    }

    /// Get next run time (simplified - not accurate)
    pub fn next_run(&self, _from: DateTime<Utc>) -> DateTime<Utc> {
        // VULNERABILITY: This is a stub - doesn't actually calculate correctly
        Utc::now() + chrono::Duration::minutes(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recurring_schedule() {
        let mut schedule = RecurringSchedule::new(Duration::from_secs(60))
            .max_runs(5);

        assert!(schedule.has_more_runs());

        for _ in 0..5 {
            schedule.run_count += 1;
        }

        assert!(!schedule.has_more_runs());
    }

    #[test]
    fn test_cron_parse_simple() {
        let cron = CronExpression::parse("* * * * *");
        assert!(matches!(cron.minute, CronField::Any));
    }

    #[test]
    #[should_panic]
    fn test_cron_parse_invalid() {
        // This will panic due to insufficient parts
        CronExpression::parse("invalid");
    }
}
