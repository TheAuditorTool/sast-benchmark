//! Job priority definitions
//!
//! Priority determines the order in which jobs are processed.
//! Higher priority jobs are processed before lower priority ones.

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// Job priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Priority {
    /// Lowest priority - processed last
    Low = 1,
    /// Normal priority - default
    Normal = 5,
    /// High priority - processed before normal
    High = 10,
    /// Critical priority - processed first
    Critical = 100,
}

impl Priority {
    /// Get the numeric value
    pub fn value(&self) -> i32 {
        *self as i32
    }

    /// Create from numeric value
    pub fn from_value(value: i32) -> Self {
        match value {
            v if v >= 100 => Self::Critical,
            v if v >= 10 => Self::High,
            v if v >= 5 => Self::Normal,
            _ => Self::Low,
        }
    }

    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Normal => "normal",
            Self::High => "high",
            Self::Critical => "critical",
        }
    }

    /// Parse from string
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "low" | "1" => Some(Self::Low),
            "normal" | "5" | "medium" => Some(Self::Normal),
            "high" | "10" => Some(Self::High),
            "critical" | "100" | "urgent" => Some(Self::Critical),
            _ => None,
        }
    }
}

impl Default for Priority {
    fn default() -> Self {
        Self::Normal
    }
}

impl Ord for Priority {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl From<i32> for Priority {
    fn from(value: i32) -> Self {
        Self::from_value(value)
    }
}

impl From<Priority> for i32 {
    fn from(priority: Priority) -> Self {
        priority.value()
    }
}

/// Priority queue implementation
#[derive(Debug)]
pub struct PriorityQueue<T> {
    items: Vec<(Priority, T)>,
}

impl<T> PriorityQueue<T> {
    /// Create a new priority queue
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Add an item with priority
    pub fn push(&mut self, priority: Priority, item: T) {
        let pos = self.items
            .iter()
            .position(|(p, _)| p < &priority)
            .unwrap_or(self.items.len());
        self.items.insert(pos, (priority, item));
    }

    /// Remove and return the highest priority item
    pub fn pop(&mut self) -> Option<T> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.items.remove(0).1)
        }
    }

    /// Peek at the highest priority item
    pub fn peek(&self) -> Option<&T> {
        self.items.first().map(|(_, item)| item)
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get the number of items
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Get items by priority
    pub fn iter_by_priority(&self, priority: Priority) -> impl Iterator<Item = &T> {
        self.items
            .iter()
            .filter(move |(p, _)| *p == priority)
            .map(|(_, item)| item)
    }
}

impl<T> Default for PriorityQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FromIterator<(Priority, T)> for PriorityQueue<T> {
    fn from_iter<I: IntoIterator<Item = (Priority, T)>>(iter: I) -> Self {
        let mut queue = Self::new();
        for (priority, item) in iter {
            queue.push(priority, item);
        }
        queue
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_ordering() {
        assert!(Priority::Critical > Priority::High);
        assert!(Priority::High > Priority::Normal);
        assert!(Priority::Normal > Priority::Low);
    }

    #[test]
    fn test_priority_queue() {
        let mut queue: PriorityQueue<&str> = PriorityQueue::new();

        queue.push(Priority::Normal, "normal job");
        queue.push(Priority::Critical, "critical job");
        queue.push(Priority::Low, "low job");
        queue.push(Priority::High, "high job");

        assert_eq!(queue.pop(), Some("critical job"));
        assert_eq!(queue.pop(), Some("high job"));
        assert_eq!(queue.pop(), Some("normal job"));
        assert_eq!(queue.pop(), Some("low job"));
        assert_eq!(queue.pop(), None);
    }
}
