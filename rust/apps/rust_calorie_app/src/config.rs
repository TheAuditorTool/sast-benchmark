//! Application configuration - Environment variables and settings.
//!
//! SECURITY: Contains sensitive configuration like JWT secrets.
//! These are NOT taint sources (they come from environment, not HTTP).

use std::env;

/// Application configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Database URL (SQLite file path)
    pub database_url: String,

    /// JWT secret for signing tokens
    pub jwt_secret: String,

    /// JWT expiration time in hours
    pub jwt_expiration_hours: i64,

    /// Server host
    pub host: String,

    /// Server port
    pub port: u16,

    /// Environment (development, production)
    pub environment: String,

    /// Log level
    pub log_level: String,

    /// bcrypt cost factor
    pub bcrypt_cost: u32,
}

impl AppConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        // Load .env file if present
        dotenv::dotenv().ok();

        Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite:calorie_app.db".to_string()),

            // vuln-code-snippet start infodisclosureCalorieJwtSecret
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| {
                    // In development, use a default (NOT for production!)
                    eprintln!("WARNING: Using default JWT_SECRET. Set JWT_SECRET env var in production!");
                    // vuln-code-snippet vuln-line infodisclosureCalorieJwtSecret
                    "super-secret-jwt-key-change-in-production".to_string()
                }),
            // vuln-code-snippet end infodisclosureCalorieJwtSecret

            jwt_expiration_hours: env::var("JWT_EXPIRATION_HOURS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(24),

            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),

            port: env::var("PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(8080),

            environment: env::var("ENVIRONMENT")
                .unwrap_or_else(|_| "development".to_string()),

            log_level: env::var("LOG_LEVEL")
                .unwrap_or_else(|_| "info".to_string()),

            bcrypt_cost: env::var("BCRYPT_COST")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(12),
        }
    }

    /// Check if running in production
    pub fn is_production(&self) -> bool {
        self.environment == "production"
    }

    /// Get server address
    pub fn server_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self::from_env()
    }
}
