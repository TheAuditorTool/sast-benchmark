//! API route handlers.

use std::sync::Arc;

use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

use jobqueue_core::{Job, JobFilter, JobId, JobState, QueueName};
use jobqueue_db::SqliteStore;

use crate::error::ApiError;
use crate::server::AppState;

// ============================================================================
// Request/Response Types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct CreateJobRequest {
    pub job_type: String,
    pub queue: Option<String>,
    pub data: serde_json::Value,
    pub priority: Option<i32>,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct JobResponse {
    pub id: String,
    pub queue: String,
    pub state: String,
    pub job_type: String,
    pub priority: i32,
    pub attempt: u32,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Job> for JobResponse {
    fn from(job: Job) -> Self {
        Self {
            id: job.id.to_string(),
            queue: job.queue.to_string(),
            state: format!("{:?}", job.state).to_lowercase(),
            job_type: job.payload.job_type,
            priority: job.priority.value(),
            attempt: job.attempt,
            created_at: job.created_at.to_rfc3339(),
            updated_at: job.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ListJobsQuery {
    pub queue: Option<String>,
    pub state: Option<String>,
    pub job_type: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct SqlQuery {
    pub sql: String,
}

// ============================================================================
// Job Handlers
// ============================================================================

/// Create a new job
pub async fn create_job(
    Extension(state): Extension<Arc<AppState>>,
    Json(req): Json<CreateJobRequest>,
) -> Result<Json<JobResponse>, ApiError> {
    let job = Job::builder(&req.job_type)
        .data_raw(req.data)
        .queue(req.queue.unwrap_or_else(|| "default".to_string()));

    let job = if let Some(priority) = req.priority {
        job.priority(jobqueue_core::Priority::from_value(priority))
    } else {
        job
    };

    let job = req.tags.into_iter().fold(job, |j, tag| j.tag(tag));
    let job = job.build();

    state.store.save(&job).await
        .map_err(|e| ApiError::internal(e.to_string()))?;

    Ok(Json(JobResponse::from(job)))
}

/// List jobs
pub async fn list_jobs(
    Extension(state): Extension<Arc<AppState>>,
    Query(query): Query<ListJobsQuery>,
) -> Result<Json<Vec<JobResponse>>, ApiError> {
    let mut filter = JobFilter::new();

    if let Some(queue) = query.queue {
        filter = filter.queue(queue);
    }

    if let Some(state_str) = query.state {
        let state = match state_str.as_str() {
            "pending" => Some(JobState::Pending),
            "running" => Some(JobState::Running),
            "completed" => Some(JobState::Completed),
            "failed" => Some(JobState::Failed),
            _ => None,
        };
        if let Some(s) = state {
            filter = filter.state(s);
        }
    }

    if let Some(job_type) = query.job_type {
        filter = filter.job_type(job_type);
    }

    if let Some(limit) = query.limit {
        filter = filter.limit(limit);
    }

    if let Some(offset) = query.offset {
        filter = filter.offset(offset);
    }

    let jobs = state.store.list(filter).await
        .map_err(|e| ApiError::internal(e.to_string()))?;

    Ok(Json(jobs.into_iter().map(JobResponse::from).collect()))
}

/// Get a job by ID
pub async fn get_job(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<JobResponse>, ApiError> {
    let job_id = JobId::from(id.clone());

    let job = state.store.get(&job_id).await
        .map_err(|e| ApiError::internal(e.to_string()))?
        .ok_or_else(|| ApiError::not_found(format!("Job not found: {}", id)))?;

    Ok(Json(JobResponse::from(job)))
}

/// Cancel a job
pub async fn cancel_job(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    let job_id = JobId::from(id);

    state.store.update_state(&job_id, JobState::Cancelled).await
        .map_err(|e| ApiError::internal(e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}

/// Retry a failed job
pub async fn retry_job(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    let job_id = JobId::from(id);

    state.store.update_state(&job_id, JobState::Pending).await
        .map_err(|e| ApiError::internal(e.to_string()))?;

    Ok(StatusCode::ACCEPTED)
}

// ============================================================================
// Queue Handlers
// ============================================================================

/// List all queues
pub async fn list_queues(
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<Vec<String>>, ApiError> {
    // Simplified - would normally query distinct queues
    Ok(Json(vec!["default".to_string()]))
}

/// Get queue statistics
pub async fn queue_stats(
    Extension(state): Extension<Arc<AppState>>,
    Path(name): Path<String>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let queue = QueueName::from(name);

    let stats = state.store.queue_stats(&queue).await
        .map_err(|e| ApiError::internal(e.to_string()))?;

    Ok(Json(serde_json::to_value(stats).unwrap()))
}

/// Pause a queue
pub async fn pause_queue(
    Extension(state): Extension<Arc<AppState>>,
    Path(name): Path<String>,
) -> Result<StatusCode, ApiError> {
    // Would implement queue pausing
    Ok(StatusCode::ACCEPTED)
}

/// Resume a queue
pub async fn resume_queue(
    Extension(state): Extension<Arc<AppState>>,
    Path(name): Path<String>,
) -> Result<StatusCode, ApiError> {
    // Would implement queue resuming
    Ok(StatusCode::ACCEPTED)
}

// ============================================================================
// Admin Handlers (VULNERABLE)
// ============================================================================

/// Search jobs
///
///SQL injection via search term
// vuln-code-snippet start sqliJobqueueHandlerSearch
pub async fn search_jobs(
    Extension(state): Extension<Arc<AppState>>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<Vec<JobResponse>>, ApiError> {
    // TAINT SINK: User input passed directly to SQL
    let jobs = state.store.search_jobs(&query.q, query.limit.unwrap_or(100)).await // vuln-code-snippet target-line sqliJobqueueHandlerSearch
        .map_err(|e| ApiError::internal(e.to_string()))?;

    Ok(Json(jobs.into_iter().map(JobResponse::from).collect()))
}
// vuln-code-snippet end sqliJobqueueHandlerSearch

/// Cleanup old jobs
pub async fn cleanup_jobs(
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let deleted = state.store.cleanup(chrono::Duration::days(30)).await
        .map_err(|e| ApiError::internal(e.to_string()))?;

    Ok(Json(serde_json::json!({
        "deleted": deleted
    })))
}

/// Execute raw SQL
///
///Direct SQL execution without authorization
// vuln-code-snippet start sqliJobqueueHandlerExecuteSql
pub async fn execute_sql(
    Extension(state): Extension<Arc<AppState>>,
    Json(query): Json<SqlQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // TAINT SINK: Arbitrary SQL execution
    // No authentication or authorization check!
    let rows = state.store.execute_raw(&query.sql).await // vuln-code-snippet target-line sqliJobqueueHandlerExecuteSql
        .map_err(|e| ApiError::internal(e.to_string()))?;

    Ok(Json(serde_json::json!({
        "rows_affected": rows,
        "sql": query.sql  //Reflects user input (potential XSS)
    })))
}
// vuln-code-snippet end sqliJobqueueHandlerExecuteSql

// ============================================================================
// Health Handlers
// ============================================================================

/// Health check
pub async fn health_check(
    Extension(state): Extension<Arc<AppState>>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

/// Liveness probe
pub async fn liveness() -> StatusCode {
    StatusCode::OK
}

/// Readiness probe
pub async fn readiness(
    Extension(state): Extension<Arc<AppState>>,
) -> StatusCode {
    // Would check database connectivity, etc.
    StatusCode::OK
}

/// Prometheus metrics
pub async fn prometheus_metrics(
    Extension(state): Extension<Arc<AppState>>,
) -> String {
    state.metrics.to_prometheus()
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_response_from() {
        let job = Job::builder("test_type").build();
        let response = JobResponse::from(job);

        assert_eq!(response.job_type, "test_type");
        assert_eq!(response.state, "pending");
    }
}
