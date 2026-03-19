//! Common type definitions
//!
//! This module contains type aliases and newtypes used throughout the library.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Deref;

/// Timestamp type alias
pub type Timestamp = DateTime<Utc>;

/// Worker identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WorkerId(String);

impl WorkerId {
    /// Create a new worker ID
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Generate a random worker ID
    pub fn generate() -> Self {
        use std::process;
        Self(format!(
            "worker-{}-{}",
            process::id(),
            uuid::Uuid::new_v4().to_string().split('-').next().unwrap()
        ))
    }

    /// Get as string slice
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for WorkerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for WorkerId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for WorkerId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl Deref for WorkerId {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Queue name
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct QueueName(String);

impl QueueName {
    /// Create a new queue name
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    /// Get as string slice
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Validate the queue name
    pub fn is_valid(&self) -> bool {
        !self.0.is_empty()
            && self.0.len() <= 256
            && self.0.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == '.')
    }
}

impl Default for QueueName {
    fn default() -> Self {
        Self(crate::DEFAULT_QUEUE.to_string())
    }
}

impl fmt::Display for QueueName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for QueueName {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for QueueName {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl Deref for QueueName {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Duration wrapper with serialization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Millis(pub u64);

impl Millis {
    pub fn from_secs(secs: u64) -> Self {
        Self(secs * 1000)
    }

    pub fn from_mins(mins: u64) -> Self {
        Self(mins * 60 * 1000)
    }

    pub fn as_duration(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.0)
    }
}

impl From<std::time::Duration> for Millis {
    fn from(d: std::time::Duration) -> Self {
        Self(d.as_millis() as u64)
    }
}

impl From<Millis> for std::time::Duration {
    fn from(m: Millis) -> Self {
        Self::from_millis(m.0)
    }
}

/// Byte size wrapper
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ByteSize(pub u64);

impl ByteSize {
    pub const KB: u64 = 1024;
    pub const MB: u64 = 1024 * 1024;
    pub const GB: u64 = 1024 * 1024 * 1024;

    pub fn kb(n: u64) -> Self {
        Self(n * Self::KB)
    }

    pub fn mb(n: u64) -> Self {
        Self(n * Self::MB)
    }

    pub fn gb(n: u64) -> Self {
        Self(n * Self::GB)
    }

    pub fn as_bytes(&self) -> u64 {
        self.0
    }
}

impl fmt::Display for ByteSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 >= Self::GB {
            write!(f, "{:.2} GB", self.0 as f64 / Self::GB as f64)
        } else if self.0 >= Self::MB {
            write!(f, "{:.2} MB", self.0 as f64 / Self::MB as f64)
        } else if self.0 >= Self::KB {
            write!(f, "{:.2} KB", self.0 as f64 / Self::KB as f64)
        } else {
            write!(f, "{} B", self.0)
        }
    }
}

/// Percentage value (0-100)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Percent(u8);

impl Percent {
    pub fn new(value: u8) -> Self {
        Self(value.min(100))
    }

    pub fn value(&self) -> u8 {
        self.0
    }

    pub fn as_fraction(&self) -> f64 {
        self.0 as f64 / 100.0
    }
}

impl fmt::Display for Percent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}%", self.0)
    }
}

impl From<u8> for Percent {
    fn from(v: u8) -> Self {
        Self::new(v)
    }
}

/// Counter for statistics
#[derive(Debug, Default)]
pub struct Counter {
    value: std::sync::atomic::AtomicU64,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            value: std::sync::atomic::AtomicU64::new(0),
        }
    }

    pub fn increment(&self) {
        self.value.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn add(&self, n: u64) {
        self.value.fetch_add(n, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn get(&self) -> u64 {
        self.value.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn reset(&self) -> u64 {
        self.value.swap(0, std::sync::atomic::Ordering::Relaxed)
    }
}

/// Rate tracker
#[derive(Debug)]
pub struct RateTracker {
    window_start: std::sync::Mutex<std::time::Instant>,
    count: Counter,
    window_seconds: u64,
}

impl RateTracker {
    pub fn new(window_seconds: u64) -> Self {
        Self {
            window_start: std::sync::Mutex::new(std::time::Instant::now()),
            count: Counter::new(),
            window_seconds,
        }
    }

    pub fn record(&self) {
        self.count.increment();
    }

    pub fn rate(&self) -> f64 {
        let start = self.window_start.lock().unwrap();
        let elapsed = start.elapsed().as_secs_f64();
        if elapsed == 0.0 {
            0.0
        } else {
            self.count.get() as f64 / elapsed
        }
    }

    pub fn reset(&self) {
        let mut start = self.window_start.lock().unwrap();
        *start = std::time::Instant::now();
        self.count.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue_name_validation() {
        assert!(QueueName::new("default").is_valid());
        assert!(QueueName::new("my-queue").is_valid());
        assert!(QueueName::new("queue_123").is_valid());
        assert!(!QueueName::new("").is_valid());
        assert!(!QueueName::new("queue with spaces").is_valid());
    }

    #[test]
    fn test_byte_size_display() {
        assert_eq!(format!("{}", ByteSize(512)), "512 B");
        assert_eq!(format!("{}", ByteSize::kb(2)), "2.00 KB");
        assert_eq!(format!("{}", ByteSize::mb(5)), "5.00 MB");
    }

    #[test]
    fn test_counter() {
        let counter = Counter::new();
        counter.increment();
        counter.increment();
        counter.add(3);
        assert_eq!(counter.get(), 5);
        assert_eq!(counter.reset(), 5);
        assert_eq!(counter.get(), 0);
    }
}
