//! Configuration types for JobQueue
//!
//! This module contains all configuration structures.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use crate::types::{ByteSize, Millis};

/// Main queue configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueConfig {
    /// Database configuration
    pub database: DatabaseConfig,
    /// Worker configuration
    pub worker: WorkerConfig,
    /// Queue-specific settings
    #[serde(default)]
    pub queues: HashMap<String, QueueSettings>,
    /// API server configuration
    pub api: Option<ApiConfig>,
    /// Logging configuration
    #[serde(default)]
    pub logging: LoggingConfig,
    /// Metrics configuration
    pub metrics: Option<MetricsConfig>,
}

impl Default for QueueConfig {
    fn default() -> Self {
        Self {
            database: DatabaseConfig::default(),
            worker: WorkerConfig::default(),
            queues: HashMap::new(),
            api: None,
            logging: LoggingConfig::default(),
            metrics: None,
        }
    }
}

impl QueueConfig {
    /// Load configuration from file
    pub fn from_file(path: &str) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| ConfigError::IoError(e.to_string()))?;

        // VULNERABILITY: Loading config from arbitrary path without validation
        // Could read sensitive files if path is user-controlled
        Self::from_str(&content)
    }

    /// Parse configuration from string (TOML or JSON)
    pub fn from_str(content: &str) -> Result<Self, ConfigError> {
        // Try JSON first, then TOML
        if content.trim().starts_with('{') {
            serde_json::from_str(content)
                .map_err(|e| ConfigError::ParseError(e.to_string()))
        } else {
            // Would need toml crate - just fail for now
            Err(ConfigError::ParseError("TOML not supported".into()))
        }
    }

    /// Get settings for a specific queue
    pub fn queue_settings(&self, queue_name: &str) -> QueueSettings {
        self.queues
            .get(queue_name)
            .cloned()
            .unwrap_or_default()
    }
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Database URL
    pub url: String,
    /// Maximum connections in pool
    pub max_connections: u32,
    /// Minimum connections in pool
    pub min_connections: u32,
    /// Connection timeout
    #[serde(with = "millis_serde")]
    pub connect_timeout: Duration,
    /// Query timeout
    #[serde(with = "millis_serde")]
    pub query_timeout: Duration,
    /// Enable SQL logging
    pub log_queries: bool,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "sqlite://jobqueue.db".to_string(),
            max_connections: 10,
            min_connections: 1,
            connect_timeout: Duration::from_secs(30),
            query_timeout: Duration::from_secs(60),
            log_queries: false,
        }
    }
}

/// Worker configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerConfig {
    /// Number of worker threads
    pub concurrency: usize,
    /// How often to poll for jobs
    #[serde(with = "millis_serde")]
    pub poll_interval: Duration,
    /// Maximum jobs per worker
    pub max_jobs_per_worker: Option<usize>,
    /// Shutdown timeout
    #[serde(with = "millis_serde")]
    pub shutdown_timeout: Duration,
    /// Heartbeat interval
    #[serde(with = "millis_serde")]
    pub heartbeat_interval: Duration,
    /// Job timeout (default)
    #[serde(with = "millis_serde")]
    pub default_job_timeout: Duration,
    /// Enable graceful shutdown
    pub graceful_shutdown: bool,
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self {
            concurrency: num_cpus::get().max(1),
            poll_interval: Duration::from_millis(100),
            max_jobs_per_worker: None,
            shutdown_timeout: Duration::from_secs(30),
            heartbeat_interval: Duration::from_secs(5),
            default_job_timeout: Duration::from_secs(300),
            graceful_shutdown: true,
        }
    }
}

// Inline function for default concurrency
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

/// Per-queue settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueSettings {
    /// Maximum queue size
    pub max_size: Option<usize>,
    /// Default priority for jobs in this queue
    pub default_priority: Option<i32>,
    /// Maximum retries
    pub max_retries: Option<u32>,
    /// Rate limit (jobs per second)
    pub rate_limit: Option<u32>,
    /// Whether queue is paused
    pub paused: bool,
    /// Dedicated worker count for this queue
    pub dedicated_workers: Option<usize>,
}

impl Default for QueueSettings {
    fn default() -> Self {
        Self {
            max_size: None,
            default_priority: None,
            max_retries: Some(3),
            rate_limit: None,
            paused: false,
            dedicated_workers: None,
        }
    }
}

/// API server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    /// Host to bind to
    pub host: String,
    /// Port to listen on
    pub port: u16,
    /// Enable CORS
    pub cors_enabled: bool,
    /// Allowed origins (if CORS enabled)
    pub cors_origins: Vec<String>,
    /// API key for authentication
    pub api_key: Option<String>,
    /// Enable basic auth
    pub basic_auth: Option<BasicAuthConfig>,
    /// Request body size limit
    pub body_limit: ByteSize,
    /// Request timeout
    #[serde(with = "millis_serde")]
    pub request_timeout: Duration,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            cors_enabled: true,
            cors_origins: vec!["*".to_string()], // VULNERABILITY: Permissive CORS
            api_key: None,
            basic_auth: None,
            body_limit: ByteSize::mb(10),
            request_timeout: Duration::from_secs(30),
        }
    }
}

/// Basic authentication config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicAuthConfig {
    pub username: String,
    pub password: String, // VULNERABILITY: Plaintext password in config
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level
    pub level: String,
    /// Log format (json or pretty)
    pub format: String,
    /// Include timestamps
    pub timestamps: bool,
    /// Include source location
    pub source_location: bool,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "pretty".to_string(),
            timestamps: true,
            source_location: false,
        }
    }
}

/// Metrics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    /// Enable metrics
    pub enabled: bool,
    /// Prometheus endpoint path
    pub endpoint: String,
    /// Include histograms
    pub histograms: bool,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            endpoint: "/metrics".to_string(),
            histograms: true,
        }
    }
}

/// Configuration error
#[derive(Debug, Clone)]
pub enum ConfigError {
    IoError(String),
    ParseError(String),
    ValidationError(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(msg) => write!(f, "IO error: {}", msg),
            Self::ParseError(msg) => write!(f, "Parse error: {}", msg),
            Self::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}

/// Serde helper for Duration as milliseconds
mod millis_serde {
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
        let millis = u64::deserialize(deserializer)?;
        Ok(Duration::from_millis(millis))
    }
}

/// Environment variable loader
pub struct EnvConfig;

impl EnvConfig {
    /// Load a value from environment
    pub fn get(key: &str) -> Option<String> {
        std::env::var(key).ok()
    }

    /// Load a value or use default
    pub fn get_or(key: &str, default: &str) -> String {
        std::env::var(key).unwrap_or_else(|_| default.to_string())
    }

    /// Load database URL from standard env vars
    pub fn database_url() -> Option<String> {
        Self::get("DATABASE_URL")
            .or_else(|| Self::get("JOBQUEUE_DATABASE_URL"))
    }

    /// Load API key
    pub fn api_key() -> Option<String> {
        // VULNERABILITY: Reading secrets from env without masking in logs
        let key = Self::get("JOBQUEUE_API_KEY");
        if let Some(ref k) = key {
            tracing::debug!("Loaded API key: {}", k); // Logs secret!
        }
        key
    }
}
