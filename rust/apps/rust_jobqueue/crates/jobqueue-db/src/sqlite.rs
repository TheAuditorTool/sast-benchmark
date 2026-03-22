//! SQLite implementation of JobStore.

use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use rusqlite::{params, Connection, OptionalExtension};
use tokio::sync::Mutex;

use jobqueue_core::{
    Job, JobBuilder, JobFilter, JobId, JobState, JobStore, Priority,
    QueueName, Result, SortOrder, WorkerId,
};

use crate::{DbError, DbResult};
use crate::migrations::run_migrations;

/// SQLite-based job store
pub struct SqliteStore {
    conn: Arc<Mutex<Connection>>,
    path: String,
}

impl SqliteStore {
    /// Create a new SQLite store
    pub fn new(path: impl AsRef<Path>) -> DbResult<Self> {
        let path_str = path.as_ref().to_string_lossy().to_string();
        let conn = Connection::open(&path_str)?;

        // Configure SQLite for performance
        conn.execute_batch(
            "PRAGMA journal_mode=WAL;
             PRAGMA synchronous=NORMAL;
             PRAGMA cache_size=10000;
             PRAGMA temp_store=memory;"
        )?;

        let store = Self {
            conn: Arc::new(Mutex::new(conn)),
            path: path_str,
        };

        // Run migrations synchronously for initialization
        let conn = store.conn.blocking_lock();
        run_migrations(&conn)?;
        drop(conn);

        Ok(store)
    }

    /// Create an in-memory store (for testing)
    pub fn in_memory() -> DbResult<Self> {
        Self::new(":memory:")
    }

    /// Get raw connection (for advanced queries)
    pub async fn connection(&self) -> tokio::sync::MutexGuard<'_, Connection> {
        self.conn.lock().await
    }

    /// Execute raw SQL (DANGEROUS - for admin use only)
    ///
    ///Allows arbitrary SQL execution
    // vuln-code-snippet start sqliJobqueueExecuteRaw
    pub async fn execute_raw(&self, sql: &str) -> DbResult<usize> {
        let conn = self.conn.lock().await;
        let rows = conn.execute(sql, [])?; // vuln-code-snippet target-line sqliJobqueueExecuteRaw
        Ok(rows)
    }
    // vuln-code-snippet end sqliJobqueueExecuteRaw

    /// Search jobs with user-provided query
    ///
    ///SQL INJECTION
    /// The search_term is directly interpolated into SQL without sanitization
    // vuln-code-snippet start sqliJobqueueSearchJobs
    pub async fn search_jobs(&self, search_term: &str, limit: usize) -> DbResult<Vec<Job>> {
        let conn = self.conn.lock().await;

        // TAINT SINK: User input directly in SQL query
        // Attacker payload: ' OR '1'='1' --
        let sql = format!(
            "SELECT * FROM jobs WHERE payload LIKE '%{}%' OR queue LIKE '%{}%' LIMIT {}", // vuln-code-snippet target-line sqliJobqueueSearchJobs
            search_term, search_term, limit
        );

        tracing::debug!("Executing search query: {}", sql);

        let mut stmt = conn.prepare(&sql)?;
        let jobs = stmt
            .query_map([], |row| self.row_to_job(row))?
            .filter_map(|r| r.ok())
            .collect();

        Ok(jobs)
    }
    // vuln-code-snippet end sqliJobqueueSearchJobs

    /// Get jobs by tag
    ///
    ///SQL INJECTION via tag name
    // vuln-code-snippet start sqliJobqueueGetJobsByTag
    pub async fn get_jobs_by_tag(&self, tag: &str) -> DbResult<Vec<Job>> {
        let conn = self.conn.lock().await;

        // TAINT SINK: Tag directly in SQL
        let sql = format!(
            "SELECT * FROM jobs WHERE tags LIKE '%\"{}\",%' OR tags LIKE '%\"{}\"]%'", // vuln-code-snippet target-line sqliJobqueueGetJobsByTag
            tag, tag
        );

        let mut stmt = conn.prepare(&sql)?;
        let jobs = stmt
            .query_map([], |row| self.row_to_job(row))?
            .filter_map(|r| r.ok())
            .collect();

        Ok(jobs)
    }
    // vuln-code-snippet end sqliJobqueueGetJobsByTag

    /// Dynamic order by
    ///
    ///SQL injection via order_by parameter
    // vuln-code-snippet start sqliJobqueueListOrdered
    pub async fn list_jobs_ordered(&self, order_by: &str, desc: bool) -> DbResult<Vec<Job>> {
        let conn = self.conn.lock().await;

        let direction = if desc { "DESC" } else { "ASC" };
        // TAINT SINK: order_by directly in SQL
        let sql = format!(
            "SELECT * FROM jobs ORDER BY {} {} LIMIT 100", // vuln-code-snippet target-line sqliJobqueueListOrdered
            order_by, direction
        );

        let mut stmt = conn.prepare(&sql)?;
        let jobs = stmt
            .query_map([], |row| self.row_to_job(row))?
            .filter_map(|r| r.ok())
            .collect();

        Ok(jobs)
    }
    // vuln-code-snippet end sqliJobqueueListOrdered

    /// Get queue statistics
    pub async fn queue_stats(&self, queue: &QueueName) -> DbResult<QueueStats> {
        let conn = self.conn.lock().await;

        let total: i64 = conn.query_row(
            "SELECT COUNT(*) FROM jobs WHERE queue = ?",
            [queue.as_str()],
            |row| row.get(0),
        )?;

        let pending: i64 = conn.query_row(
            "SELECT COUNT(*) FROM jobs WHERE queue = ? AND state = 'pending'",
            [queue.as_str()],
            |row| row.get(0),
        )?;

        let running: i64 = conn.query_row(
            "SELECT COUNT(*) FROM jobs WHERE queue = ? AND state = 'running'",
            [queue.as_str()],
            |row| row.get(0),
        )?;

        let completed: i64 = conn.query_row(
            "SELECT COUNT(*) FROM jobs WHERE queue = ? AND state = 'completed'",
            [queue.as_str()],
            |row| row.get(0),
        )?;

        let failed: i64 = conn.query_row(
            "SELECT COUNT(*) FROM jobs WHERE queue = ? AND state = 'failed'",
            [queue.as_str()],
            |row| row.get(0),
        )?;

        Ok(QueueStats {
            total: total as u64,
            pending: pending as u64,
            running: running as u64,
            completed: completed as u64,
            failed: failed as u64,
        })
    }

    /// Internal: Convert row to Job
    fn row_to_job(&self, row: &rusqlite::Row) -> rusqlite::Result<Job> {
        let id: String = row.get("id")?;
        let queue: String = row.get("queue")?;
        let state_str: String = row.get("state")?;
        let priority: i32 = row.get("priority")?;
        let payload: String = row.get("payload")?;
        let retry_config: String = row.get("retry_config")?;
        let attempt: u32 = row.get("attempt")?;
        let worker_id: Option<String> = row.get("worker_id")?;
        let created_at: String = row.get("created_at")?;
        let updated_at: String = row.get("updated_at")?;
        let scheduled_at: Option<String> = row.get("scheduled_at")?;
        let started_at: Option<String> = row.get("started_at")?;
        let finished_at: Option<String> = row.get("finished_at")?;
        let timeout_ms: Option<i64> = row.get("timeout_ms")?;
        let error: Option<String> = row.get("error")?;
        let dependencies: String = row.get("dependencies")?;
        let tags: String = row.get("tags")?;
        let progress: Option<u8> = row.get("progress")?;
        let result: Option<String> = row.get("result")?;

        let state = match state_str.as_str() {
            "pending" => JobState::Pending,
            "running" => JobState::Running,
            "completed" => JobState::Completed,
            "failed" => JobState::Failed,
            "retrying" => JobState::Retrying,
            "cancelled" => JobState::Cancelled,
            "scheduled" => JobState::Scheduled,
            "blocked" => JobState::Blocked,
            _ => JobState::Pending,
        };

        let parse_datetime = |s: &str| -> DateTime<Utc> {
            DateTime::parse_from_rfc3339(s)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now())
        };

        Ok(Job {
            id: JobId::from(id),
            queue: QueueName::from(queue),
            state,
            priority: Priority::from_value(priority),
            payload: serde_json::from_str(&payload).unwrap_or_else(|_| {
                jobqueue_core::JobPayload {
                    job_type: "unknown".to_string(),
                    data: serde_json::Value::Null,
                    metadata: std::collections::HashMap::new(),
                }
            }),
            retry_config: serde_json::from_str(&retry_config).unwrap_or_default(),
            attempt,
            worker_id: worker_id.map(WorkerId::from),
            created_at: parse_datetime(&created_at),
            updated_at: parse_datetime(&updated_at),
            scheduled_at: scheduled_at.map(|s| parse_datetime(&s)),
            started_at: started_at.map(|s| parse_datetime(&s)),
            finished_at: finished_at.map(|s| parse_datetime(&s)),
            timeout: timeout_ms.map(|ms| std::time::Duration::from_millis(ms as u64)),
            error,
            dependencies: serde_json::from_str(&dependencies).unwrap_or_default(),
            tags: serde_json::from_str(&tags).unwrap_or_default(),
            progress,
            result: result.and_then(|s| serde_json::from_str(&s).ok()),
        })
    }

    /// Internal: Convert Job to params
    fn job_to_params(&self, job: &Job) -> (
        String, String, String, i32, String, String, u32,
        Option<String>, String, String, Option<String>,
        Option<String>, Option<String>, Option<i64>,
        Option<String>, String, String, Option<u8>, Option<String>
    ) {
        let state = match job.state {
            JobState::Pending => "pending",
            JobState::Running => "running",
            JobState::Completed => "completed",
            JobState::Failed => "failed",
            JobState::Retrying => "retrying",
            JobState::Cancelled => "cancelled",
            JobState::Scheduled => "scheduled",
            JobState::Blocked => "blocked",
        };

        (
            job.id.to_string(),
            job.queue.to_string(),
            state.to_string(),
            job.priority.value(),
            serde_json::to_string(&job.payload).unwrap_or_default(),
            serde_json::to_string(&job.retry_config).unwrap_or_default(),
            job.attempt,
            job.worker_id.as_ref().map(|w| w.to_string()),
            job.created_at.to_rfc3339(),
            job.updated_at.to_rfc3339(),
            job.scheduled_at.map(|dt| dt.to_rfc3339()),
            job.started_at.map(|dt| dt.to_rfc3339()),
            job.finished_at.map(|dt| dt.to_rfc3339()),
            job.timeout.map(|d| d.as_millis() as i64),
            job.error.clone(),
            serde_json::to_string(&job.dependencies).unwrap_or_default(),
            serde_json::to_string(&job.tags).unwrap_or_default(),
            job.progress,
            job.result.as_ref().map(|r| serde_json::to_string(r).unwrap_or_default()),
        )
    }
}

#[async_trait]
impl JobStore for SqliteStore {
    async fn save(&self, job: &Job) -> Result<()> {
        let conn = self.conn.lock().await;

        let (
            id, queue, state, priority, payload, retry_config, attempt,
            worker_id, created_at, updated_at, scheduled_at, started_at,
            finished_at, timeout_ms, error, dependencies, tags, progress, result
        ) = self.job_to_params(job);

        conn.execute(
            "INSERT OR REPLACE INTO jobs (
                id, queue, state, priority, payload, retry_config, attempt,
                worker_id, created_at, updated_at, scheduled_at, started_at,
                finished_at, timeout_ms, error, dependencies, tags, progress, result
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                id, queue, state, priority, payload, retry_config, attempt,
                worker_id, created_at, updated_at, scheduled_at, started_at,
                finished_at, timeout_ms, error, dependencies, tags, progress, result
            ],
        ).map_err(|e| jobqueue_core::JobQueueError::database(e.to_string(), None))?;

        Ok(())
    }

    async fn get(&self, id: &JobId) -> Result<Option<Job>> {
        let conn = self.conn.lock().await;

        let job = conn
            .query_row(
                "SELECT * FROM jobs WHERE id = ?",
                [id.as_str()],
                |row| self.row_to_job(row),
            )
            .optional()
            .map_err(|e| jobqueue_core::JobQueueError::database(e.to_string(), None))?;

        Ok(job)
    }

    async fn delete(&self, id: &JobId) -> Result<bool> {
        let conn = self.conn.lock().await;

        let rows = conn
            .execute("DELETE FROM jobs WHERE id = ?", [id.as_str()])
            .map_err(|e| jobqueue_core::JobQueueError::database(e.to_string(), None))?;

        Ok(rows > 0)
    }

    async fn update_state(&self, id: &JobId, state: JobState) -> Result<()> {
        let conn = self.conn.lock().await;

        let state_str = match state {
            JobState::Pending => "pending",
            JobState::Running => "running",
            JobState::Completed => "completed",
            JobState::Failed => "failed",
            JobState::Retrying => "retrying",
            JobState::Cancelled => "cancelled",
            JobState::Scheduled => "scheduled",
            JobState::Blocked => "blocked",
        };

        conn.execute(
            "UPDATE jobs SET state = ?, updated_at = ? WHERE id = ?",
            params![state_str, Utc::now().to_rfc3339(), id.as_str()],
        ).map_err(|e| jobqueue_core::JobQueueError::database(e.to_string(), None))?;

        Ok(())
    }

    async fn list(&self, filter: JobFilter) -> Result<Vec<Job>> {
        let conn = self.conn.lock().await;

        let mut sql = String::from("SELECT * FROM jobs WHERE 1=1");
        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(ref queue) = filter.queue {
            sql.push_str(" AND queue = ?");
            params_vec.push(Box::new(queue.to_string()));
        }

        if let Some(ref state) = filter.state {
            let state_str = match state {
                JobState::Pending => "pending",
                JobState::Running => "running",
                JobState::Completed => "completed",
                JobState::Failed => "failed",
                JobState::Retrying => "retrying",
                JobState::Cancelled => "cancelled",
                JobState::Scheduled => "scheduled",
                JobState::Blocked => "blocked",
            };
            sql.push_str(" AND state = ?");
            params_vec.push(Box::new(state_str.to_string()));
        }

        if let Some(ref job_type) = filter.job_type {
            sql.push_str(" AND json_extract(payload, '$.job_type') = ?");
            params_vec.push(Box::new(job_type.clone()));
        }

        // Add ordering
        sql.push_str(match filter.order {
            SortOrder::OldestFirst => " ORDER BY created_at ASC",
            SortOrder::NewestFirst => " ORDER BY created_at DESC",
            SortOrder::PriorityDesc => " ORDER BY priority DESC, created_at ASC",
            SortOrder::PriorityAsc => " ORDER BY priority ASC, created_at ASC",
        });

        // Add limit/offset
        if let Some(limit) = filter.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }
        if let Some(offset) = filter.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        let mut stmt = conn.prepare(&sql)
            .map_err(|e| jobqueue_core::JobQueueError::database(e.to_string(), Some(sql.clone())))?;

        let params_refs: Vec<&dyn rusqlite::ToSql> =
            params_vec.iter().map(|p| p.as_ref()).collect();

        let jobs = stmt
            .query_map(params_refs.as_slice(), |row| self.row_to_job(row))
            .map_err(|e| jobqueue_core::JobQueueError::database(e.to_string(), None))?
            .filter_map(|r| r.ok())
            .collect();

        Ok(jobs)
    }

    async fn count(&self, filter: JobFilter) -> Result<u64> {
        let jobs = self.list(filter).await?;
        Ok(jobs.len() as u64)
    }

    /// Claim the next available job
    ///
    ///Race condition - no proper locking between SELECT and UPDATE
    async fn claim_next(&self, queue: &QueueName, worker: &WorkerId) -> Result<Option<Job>> {
        let conn = self.conn.lock().await;

        //TOCTOU race condition
        // Another worker could claim the job between SELECT and UPDATE
        let job: Option<Job> = conn
            .query_row(
                "SELECT * FROM jobs
                 WHERE queue = ? AND state = 'pending'
                 ORDER BY priority DESC, created_at ASC
                 LIMIT 1",
                [queue.as_str()],
                |row| self.row_to_job(row),
            )
            .optional()
            .map_err(|e| jobqueue_core::JobQueueError::database(e.to_string(), None))?;

        if let Some(mut job) = job {
            // No row-level lock - race condition!
            conn.execute(
                "UPDATE jobs SET state = 'running', worker_id = ?, started_at = ?, updated_at = ?
                 WHERE id = ? AND state = 'pending'",
                params![
                    worker.as_str(),
                    Utc::now().to_rfc3339(),
                    Utc::now().to_rfc3339(),
                    job.id.as_str()
                ],
            ).map_err(|e| jobqueue_core::JobQueueError::database(e.to_string(), None))?;

            job.state = JobState::Running;
            job.worker_id = Some(worker.clone());
            job.started_at = Some(Utc::now());

            Ok(Some(job))
        } else {
            Ok(None)
        }
    }

    async fn release(&self, id: &JobId) -> Result<()> {
        let conn = self.conn.lock().await;

        conn.execute(
            "UPDATE jobs SET state = 'pending', worker_id = NULL, started_at = NULL, updated_at = ?
             WHERE id = ?",
            params![Utc::now().to_rfc3339(), id.as_str()],
        ).map_err(|e| jobqueue_core::JobQueueError::database(e.to_string(), None))?;

        Ok(())
    }

    async fn complete(&self, id: &JobId, result: serde_json::Value) -> Result<()> {
        let conn = self.conn.lock().await;

        let result_str = serde_json::to_string(&result)
            .map_err(|e| jobqueue_core::JobQueueError::SerializationError(e))?;

        conn.execute(
            "UPDATE jobs SET state = 'completed', finished_at = ?, updated_at = ?, result = ?
             WHERE id = ?",
            params![
                Utc::now().to_rfc3339(),
                Utc::now().to_rfc3339(),
                result_str,
                id.as_str()
            ],
        ).map_err(|e| jobqueue_core::JobQueueError::database(e.to_string(), None))?;

        Ok(())
    }

    async fn fail(&self, id: &JobId, error: &str) -> Result<()> {
        let conn = self.conn.lock().await;

        conn.execute(
            "UPDATE jobs SET state = 'failed', finished_at = ?, updated_at = ?, error = ?
             WHERE id = ?",
            params![
                Utc::now().to_rfc3339(),
                Utc::now().to_rfc3339(),
                error,
                id.as_str()
            ],
        ).map_err(|e| jobqueue_core::JobQueueError::database(e.to_string(), None))?;

        Ok(())
    }

    async fn get_retry_ready(&self) -> Result<Vec<Job>> {
        let conn = self.conn.lock().await;

        let mut stmt = conn.prepare(
            "SELECT * FROM jobs WHERE state = 'retrying' AND updated_at <= datetime('now')"
        ).map_err(|e| jobqueue_core::JobQueueError::database(e.to_string(), None))?;

        let jobs = stmt
            .query_map([], |row| self.row_to_job(row))
            .map_err(|e| jobqueue_core::JobQueueError::database(e.to_string(), None))?
            .filter_map(|r| r.ok())
            .collect();

        Ok(jobs)
    }

    async fn get_scheduled_due(&self) -> Result<Vec<Job>> {
        let conn = self.conn.lock().await;

        let mut stmt = conn.prepare(
            "SELECT * FROM jobs WHERE state = 'scheduled' AND scheduled_at <= datetime('now')"
        ).map_err(|e| jobqueue_core::JobQueueError::database(e.to_string(), None))?;

        let jobs = stmt
            .query_map([], |row| self.row_to_job(row))
            .map_err(|e| jobqueue_core::JobQueueError::database(e.to_string(), None))?
            .filter_map(|r| r.ok())
            .collect();

        Ok(jobs)
    }

    async fn cleanup(&self, older_than: Duration) -> Result<u64> {
        let conn = self.conn.lock().await;

        let cutoff = (Utc::now() - older_than).to_rfc3339();

        let rows = conn.execute(
            "DELETE FROM jobs WHERE state IN ('completed', 'failed', 'cancelled')
             AND finished_at < ?",
            [cutoff],
        ).map_err(|e| jobqueue_core::JobQueueError::database(e.to_string(), None))?;

        Ok(rows as u64)
    }
}

/// Queue statistics
#[derive(Debug, Clone, serde::Serialize)]
pub struct QueueStats {
    pub total: u64,
    pub pending: u64,
    pub running: u64,
    pub completed: u64,
    pub failed: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sqlite_store() {
        let store = SqliteStore::in_memory().unwrap();

        let job = Job::builder("test_job")
            .data_raw(serde_json::json!({"key": "value"}))
            .queue("test_queue")
            .build();

        store.save(&job).await.unwrap();

        let retrieved = store.get(&job.id).await.unwrap();
        assert!(retrieved.is_some());

        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.id, job.id);
        assert_eq!(retrieved.payload.job_type, "test_job");
    }

    #[tokio::test]
    async fn test_claim_job() {
        let store = SqliteStore::in_memory().unwrap();
        let queue = QueueName::new("test");
        let worker = WorkerId::generate();

        let job = Job::builder("task")
            .queue("test")
            .build();

        store.save(&job).await.unwrap();

        let claimed = store.claim_next(&queue, &worker).await.unwrap();
        assert!(claimed.is_some());

        let claimed = claimed.unwrap();
        assert_eq!(claimed.state, JobState::Running);
        assert_eq!(claimed.worker_id, Some(worker));
    }
}
