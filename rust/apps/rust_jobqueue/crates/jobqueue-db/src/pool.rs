//! Connection pool implementation
//!
//! Provides a simple connection pool for database connections.

use std::collections::VecDeque;
use std::sync::Arc;
use std::time::{Duration, Instant};

use rusqlite::Connection;
use tokio::sync::{Mutex, Semaphore};

use crate::{DbError, DbResult};

/// Connection pool configuration
#[derive(Debug, Clone)]
pub struct PoolConfig {
    /// Maximum number of connections
    pub max_connections: usize,
    /// Minimum number of connections to keep alive
    pub min_connections: usize,
    /// Connection timeout
    pub connect_timeout: Duration,
    /// Maximum lifetime of a connection
    pub max_lifetime: Duration,
    /// Idle timeout before closing connection
    pub idle_timeout: Duration,
    /// Database path
    pub database_path: String,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            max_connections: 10,
            min_connections: 1,
            connect_timeout: Duration::from_secs(30),
            max_lifetime: Duration::from_secs(1800),
            idle_timeout: Duration::from_secs(600),
            database_path: ":memory:".to_string(),
        }
    }
}

impl PoolConfig {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            database_path: path.into(),
            ..Default::default()
        }
    }

    pub fn max_connections(mut self, n: usize) -> Self {
        self.max_connections = n;
        self
    }

    pub fn min_connections(mut self, n: usize) -> Self {
        self.min_connections = n;
        self
    }
}

/// A pooled connection
struct PooledConnection {
    conn: Connection,
    created_at: Instant,
    last_used: Instant,
}

impl PooledConnection {
    fn new(conn: Connection) -> Self {
        let now = Instant::now();
        Self {
            conn,
            created_at: now,
            last_used: now,
        }
    }

    fn is_expired(&self, max_lifetime: Duration) -> bool {
        self.created_at.elapsed() > max_lifetime
    }

    fn is_idle(&self, idle_timeout: Duration) -> bool {
        self.last_used.elapsed() > idle_timeout
    }

    fn touch(&mut self) {
        self.last_used = Instant::now();
    }
}

/// Connection pool
pub struct ConnectionPool {
    config: PoolConfig,
    connections: Arc<Mutex<VecDeque<PooledConnection>>>,
    semaphore: Arc<Semaphore>,
}

impl ConnectionPool {
    /// Create a new connection pool
    pub fn new(config: PoolConfig) -> DbResult<Self> {
        let pool = Self {
            semaphore: Arc::new(Semaphore::new(config.max_connections)),
            config,
            connections: Arc::new(Mutex::new(VecDeque::new())),
        };

        // Pre-warm the pool with minimum connections
        {
            let mut conns = pool.connections.blocking_lock();
            for _ in 0..pool.config.min_connections {
                let conn = pool.create_connection()?;
                conns.push_back(PooledConnection::new(conn));
            }
        }

        Ok(pool)
    }

    /// Create a new database connection
    fn create_connection(&self) -> DbResult<Connection> {
        let conn = Connection::open(&self.config.database_path)?;

        // Configure connection
        conn.execute_batch(
            "PRAGMA journal_mode=WAL;
             PRAGMA synchronous=NORMAL;
             PRAGMA cache_size=10000;"
        )?;

        Ok(conn)
    }

    /// Get a connection from the pool
    pub async fn get(&self) -> DbResult<PoolGuard> {
        // Wait for available connection slot
        let permit = tokio::time::timeout(
            self.config.connect_timeout,
            self.semaphore.clone().acquire_owned(),
        )
        .await
        .map_err(|_| DbError::Connection("Connection timeout".into()))?
        .map_err(|_| DbError::PoolExhausted)?;

        let mut connections = self.connections.lock().await;

        // Try to get an existing connection
        while let Some(mut pooled) = connections.pop_front() {
            // Skip expired connections
            if pooled.is_expired(self.config.max_lifetime) {
                continue;
            }

            // Skip stale idle connections (but keep minimum)
            if pooled.is_idle(self.config.idle_timeout)
                && connections.len() >= self.config.min_connections
            {
                continue;
            }

            pooled.touch();
            return Ok(PoolGuard {
                pooled: Some(pooled),
                pool: Arc::clone(&self.connections),
                _permit: permit,
            });
        }

        // Create new connection
        let conn = self.create_connection()?;
        Ok(PoolGuard {
            pooled: Some(PooledConnection::new(conn)),
            pool: Arc::clone(&self.connections),
            _permit: permit,
        })
    }

    /// Get pool statistics
    pub async fn stats(&self) -> PoolStats {
        let connections = self.connections.lock().await;
        let available = connections.len();
        let in_use = self.config.max_connections - self.semaphore.available_permits();

        PoolStats {
            max_connections: self.config.max_connections,
            available,
            in_use,
        }
    }

    /// Close all connections
    pub async fn close(&self) {
        let mut connections = self.connections.lock().await;
        connections.clear();
    }
}

/// Guard that returns connection to pool on drop
pub struct PoolGuard {
    pooled: Option<PooledConnection>,
    pool: Arc<Mutex<VecDeque<PooledConnection>>>,
    _permit: tokio::sync::OwnedSemaphorePermit,
}

impl PoolGuard {
    /// Get reference to the connection
    pub fn conn(&self) -> &Connection {
        &self.pooled.as_ref().unwrap().conn
    }

    /// Get mutable reference to the connection
    pub fn conn_mut(&mut self) -> &mut Connection {
        &mut self.pooled.as_mut().unwrap().conn
    }
}

impl Drop for PoolGuard {
    fn drop(&mut self) {
        if let Some(pooled) = self.pooled.take() {
            // Return connection to pool
            // VULNERABILITY: Uses blocking_lock in async context
            // This can cause deadlocks under high load
            let mut connections = self.pool.blocking_lock();
            connections.push_back(pooled);
        }
    }
}

impl std::ops::Deref for PoolGuard {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        self.conn()
    }
}

/// Pool statistics
#[derive(Debug, Clone, serde::Serialize)]
pub struct PoolStats {
    pub max_connections: usize,
    pub available: usize,
    pub in_use: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pool_basic() {
        let config = PoolConfig::new(":memory:")
            .max_connections(5)
            .min_connections(1);

        let pool = ConnectionPool::new(config).unwrap();

        let conn = pool.get().await.unwrap();
        conn.execute("SELECT 1", []).unwrap();

        let stats = pool.stats().await;
        assert_eq!(stats.max_connections, 5);
    }

    #[tokio::test]
    async fn test_pool_reuse() {
        let config = PoolConfig::new(":memory:").max_connections(2);
        let pool = ConnectionPool::new(config).unwrap();

        // Get and release connection
        {
            let _conn = pool.get().await.unwrap();
        }

        // Should reuse the connection
        let stats = pool.stats().await;
        assert!(stats.available >= 1);
    }
}
