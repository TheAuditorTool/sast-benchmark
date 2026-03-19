//! API server implementation

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use axum::{
    Router,
    routing::{get, post, delete},
    Extension,
};
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;

use jobqueue_core::{Metrics, SharedMetrics};
use jobqueue_db::SqliteStore;
use jobqueue_worker::Worker;

use crate::routes;
use crate::middleware::auth_layer;

/// API server configuration
#[derive(Debug, Clone)]
pub struct ApiConfig {
    /// Host to bind to
    pub host: String,
    /// Port to listen on
    pub port: u16,
    /// Enable CORS
    pub cors_enabled: bool,
    /// API key for authentication (optional)
    pub api_key: Option<String>,
    /// Request timeout
    pub request_timeout: Duration,
    /// Enable metrics endpoint
    pub metrics_enabled: bool,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            cors_enabled: true,
            api_key: None,
            request_timeout: Duration::from_secs(30),
            metrics_enabled: true,
        }
    }
}

impl ApiConfig {
    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

/// Shared application state
pub struct AppState {
    pub store: Arc<SqliteStore>,
    pub metrics: SharedMetrics,
    pub config: ApiConfig,
}

/// API server
pub struct ApiServer {
    config: ApiConfig,
    store: Arc<SqliteStore>,
    metrics: SharedMetrics,
}

impl ApiServer {
    /// Create a new API server
    pub fn new(store: SqliteStore, config: ApiConfig) -> Self {
        Self {
            config,
            store: Arc::new(store),
            metrics: Arc::new(Metrics::new()),
        }
    }

    /// Build the router
    pub fn router(&self) -> Router {
        let state = Arc::new(AppState {
            store: Arc::clone(&self.store),
            metrics: Arc::clone(&self.metrics),
            config: self.config.clone(),
        });

        let mut router = Router::new()
            // Job routes
            .route("/jobs", post(routes::create_job))
            .route("/jobs", get(routes::list_jobs))
            .route("/jobs/:id", get(routes::get_job))
            .route("/jobs/:id", delete(routes::cancel_job))
            .route("/jobs/:id/retry", post(routes::retry_job))

            // Queue routes
            .route("/queues", get(routes::list_queues))
            .route("/queues/:name/stats", get(routes::queue_stats))
            .route("/queues/:name/pause", post(routes::pause_queue))
            .route("/queues/:name/resume", post(routes::resume_queue))

            // Admin routes
            .route("/admin/search", get(routes::search_jobs))
            .route("/admin/cleanup", post(routes::cleanup_jobs))
            .route("/admin/sql", post(routes::execute_sql)) // DANGEROUS!

            // Health and metrics
            .route("/health", get(routes::health_check))
            .route("/health/live", get(routes::liveness))
            .route("/health/ready", get(routes::readiness));

        // Add metrics endpoint if enabled
        if self.config.metrics_enabled {
            router = router.route("/metrics", get(routes::prometheus_metrics));
        }

        // Add state
        router = router.layer(Extension(state));

        // Add tracing
        router = router.layer(TraceLayer::new_for_http());

        // Add CORS if enabled
        if self.config.cors_enabled {
            // VULNERABILITY: Overly permissive CORS configuration
            let cors = CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
                .allow_credentials(false); // Even worse if true with Any origin

            router = router.layer(cors);
        }

        router
    }

    /// Start the server
    pub async fn run(&self) -> Result<(), std::io::Error> {
        let addr: SocketAddr = self.config.bind_addr()
            .parse()
            .expect("Invalid bind address");

        let router = self.router();

        tracing::info!(address = %addr, "Starting API server");

        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, router).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = ApiConfig::default();
        assert_eq!(config.port, 8080);
        assert!(config.cors_enabled);
    }

    #[test]
    fn test_bind_addr() {
        let config = ApiConfig::default();
        assert_eq!(config.bind_addr(), "0.0.0.0:8080");
    }
}
