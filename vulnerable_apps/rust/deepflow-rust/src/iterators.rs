//! Iterator chain dataflow - data flows through iterator combinators.
//!
//! This module demonstrates taint propagation through:
//! - Iterator chains (map, filter, fold, etc.)
//! - Custom iterators
//! - Lazy evaluation
//! - Collect patterns
//! - Parallel iteration

use crate::sinks;
use std::collections::HashMap;

// ============================================================================
// BASIC ITERATOR CHAINS
// ============================================================================

/// Simple map chain - taint flows through transformations
pub fn map_chain(items: Vec<String>) -> Result<String, String> {
    let result: Vec<String> = items
        .into_iter()
        .map(|s| format!("M1[{}]", s))  // Each item tainted
        .map(|s| format!("M2[{}]", s))
        .map(|s| s.to_uppercase())
        .collect();

    // TAINT SINK: All mapped items
    let joined = result.join(",");
    sinks::write_to_file("/tmp/map_chain.txt", &joined)
}

/// Filter chain - taint preserved in filtered items
pub fn filter_chain(items: Vec<String>, pattern: &str) -> Result<String, String> {
    // Pattern might be tainted too
    let result: Vec<String> = items
        .into_iter()
        .filter(|s| s.contains(pattern))  // Pattern is tainted
        .filter(|s| !s.is_empty())
        .map(|s| format!("FILTERED[{}]", s))
        .collect();

    // TAINT SINK: Filtered items
    let joined = result.join(";");
    sinks::execute_query(&format!("INSERT INTO filtered VALUES ('{}')", joined))
}

/// Fold chain - accumulates taint
pub fn fold_chain(items: Vec<String>) -> Result<String, String> {
    let result = items
        .into_iter()
        .fold(String::new(), |mut acc, item| {
            // Both acc and item carry taint
            acc.push_str(&item);
            acc.push('|');
            acc
        });

    // TAINT SINK: Folded result
    sinks::execute_shell(&format!("echo {}", result))
}

/// Flat map - taint spreads through flattening
pub fn flat_map_chain(items: Vec<String>) -> Result<String, String> {
    let result: Vec<String> = items
        .into_iter()
        .flat_map(|s| s.split(',').map(String::from).collect::<Vec<_>>())
        .map(|s| format!("FLAT[{}]", s.trim()))
        .collect();

    // TAINT SINK: Flattened items
    sinks::write_to_file("/tmp/flat_map.txt", &result.join("\n"))
}

// ============================================================================
// COMPLEX ITERATOR CHAINS
// ============================================================================

/// Long chain of operations
pub fn long_chain(items: Vec<String>) -> Result<String, String> {
    let result: String = items
        .into_iter()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .map(|s| format!("[{}]", s))
        .enumerate()
        .map(|(i, s)| format!("{}:{}", i, s))
        .filter(|s| s.len() < 100)
        .take(10)
        .fold(String::new(), |mut acc, s| {
            if !acc.is_empty() {
                acc.push(',');
            }
            acc.push_str(&s);
            acc
        });

    // TAINT SINK: Long chain result
    sinks::execute_query(&format!("INSERT INTO long_chain VALUES ('{}')", result))
}

/// Zip chains - combines two tainted sources
pub fn zip_chain(items1: Vec<String>, items2: Vec<String>) -> Result<String, String> {
    let result: Vec<String> = items1
        .into_iter()
        .zip(items2.into_iter())
        .map(|(a, b)| format!("{}+{}", a, b))  // Both a and b tainted
        .collect();

    // TAINT SINK: Zipped result
    sinks::write_to_file("/tmp/zipped.txt", &result.join("\n"))
}

/// Chain with scan - accumulator and items
pub fn scan_chain(items: Vec<String>) -> Result<String, String> {
    let result: Vec<String> = items
        .into_iter()
        .scan(0usize, |state, item| {
            *state += 1;
            Some(format!("SCAN{}[{}]", state, item))
        })
        .collect();

    // TAINT SINK: Scanned result
    sinks::fetch_url(&format!("http://api/scan?data={}", result.join(",")))
}

/// Window/chunk processing
pub fn chunk_process(items: Vec<String>, chunk_size: usize) -> Result<String, String> {
    let chunks: Vec<String> = items
        .chunks(chunk_size)
        .map(|chunk| {
            // Each chunk contains tainted items
            chunk.join("-")
        })
        .collect();

    // TAINT SINK: Chunked result
    sinks::write_to_file("/tmp/chunks.txt", &chunks.join("\n"))
}

// ============================================================================
// ITERATOR WITH SIDE EFFECTS
// ============================================================================

/// For each with sink
pub fn foreach_to_sink(items: Vec<String>) -> Result<(), String> {
    items.into_iter().for_each(|item| {
        // TAINT SINK: Each item written
        let _ = sinks::write_to_file(
            &format!("/tmp/item_{}.txt", item.len()),
            &item,
        );
    });

    Ok(())
}

/// Inspect chain - logs tainted data
pub fn inspect_chain(items: Vec<String>) -> Result<String, String> {
    let result: Vec<String> = items
        .into_iter()
        .inspect(|s| {
            // TAINT SINK: Inspection logs tainted data
            sinks::log_data("INSPECT", s);
        })
        .map(|s| format!("INSPECTED[{}]", s))
        .collect();

    sinks::write_to_file("/tmp/inspected.txt", &result.join(","))
}

// ============================================================================
// CUSTOM ITERATORS
// ============================================================================

/// Custom iterator that yields tainted items
pub struct TaintedIterator {
    items: Vec<String>,
    index: usize,
    prefix: String,
}

impl TaintedIterator {
    pub fn new(items: Vec<String>, prefix: String) -> Self {
        Self {
            items,
            index: 0,
            prefix,  // Prefix might be tainted
        }
    }
}

impl Iterator for TaintedIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.items.len() {
            let item = &self.items[self.index];
            self.index += 1;
            // Both prefix and item are tainted
            Some(format!("{}[{}]", self.prefix, item))
        } else {
            None
        }
    }
}

/// Use custom iterator
pub fn use_custom_iterator(items: Vec<String>, prefix: String) -> Result<String, String> {
    let result: Vec<String> = TaintedIterator::new(items, prefix)
        .map(|s| s.to_uppercase())
        .collect();

    // TAINT SINK: Custom iterator output
    sinks::execute_query(&format!("INSERT INTO custom VALUES ('{}')", result.join(",")))
}

/// Infinite iterator with tainted data
pub struct InfiniteGenerator {
    current: String,
    modifier: String,
}

impl InfiniteGenerator {
    pub fn new(seed: String, modifier: String) -> Self {
        Self {
            current: seed,      // Seed is tainted
            modifier: modifier, // Modifier might be tainted
        }
    }
}

impl Iterator for InfiniteGenerator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current.clone();
        self.current = format!("{}[{}]", self.modifier, self.current);
        Some(result)
    }
}

/// Take from infinite generator
pub fn take_from_infinite(seed: String, count: usize) -> Result<String, String> {
    let result: Vec<String> = InfiniteGenerator::new(seed, "GEN".to_string())
        .take(count)
        .collect();

    // TAINT SINK: Generated items
    sinks::write_to_file("/tmp/generated.txt", &result.join("\n"))
}

// ============================================================================
// ITERATOR ADAPTERS
// ============================================================================

/// Peekable iterator
pub fn peekable_process(items: Vec<String>) -> Result<String, String> {
    let mut iter = items.into_iter().peekable();
    let mut result = Vec::new();

    while let Some(item) = iter.next() {
        let next_preview = iter.peek().map(|s| s.as_str()).unwrap_or("END");
        // Both item and peeked item are tainted
        result.push(format!("{} -> {}", item, next_preview));
    }

    // TAINT SINK: Peekable result
    sinks::execute_shell(&format!("echo '{}'", result.join("; ")))
}

/// Skip/take with tainted count
pub fn skip_take(items: Vec<String>, skip: usize, take: usize) -> Result<String, String> {
    // skip and take might be user-controlled
    let result: Vec<String> = items
        .into_iter()
        .skip(skip)
        .take(take)
        .collect();

    // TAINT SINK: Skip/take result
    sinks::write_to_file("/tmp/skip_take.txt", &result.join(","))
}

/// Step by with tainted step
pub fn step_by_process(items: Vec<String>, step: usize) -> Result<String, String> {
    // step might be user-controlled
    let result: Vec<String> = items
        .into_iter()
        .step_by(step.max(1))  // Prevent step of 0
        .collect();

    // TAINT SINK: Stepped result
    sinks::fetch_url(&format!("http://api/step?data={}", result.join(",")))
}

/// Cycle with tainted data
pub fn cycle_process(items: Vec<String>, count: usize) -> Result<String, String> {
    let result: Vec<String> = items
        .into_iter()
        .cycle()
        .take(count)
        .enumerate()
        .map(|(i, s)| format!("{}:{}", i, s))
        .collect();

    // TAINT SINK: Cycled result
    sinks::write_to_file("/tmp/cycled.txt", &result.join("\n"))
}

// ============================================================================
// COLLECT PATTERNS
// ============================================================================

/// Collect to HashMap with tainted keys/values
pub fn collect_to_hashmap(items: Vec<String>) -> Result<String, String> {
    let map: HashMap<String, String> = items
        .into_iter()
        .enumerate()
        .map(|(i, s)| (format!("key_{}", i), s))  // Both key suffix and value tainted
        .collect();

    // TAINT SINK: HashMap values
    let values: String = map.values().cloned().collect::<Vec<_>>().join(",");
    sinks::execute_query(&format!("INSERT INTO map_data VALUES ('{}')", values))
}

/// Collect with partition
pub fn partition_and_sink(items: Vec<String>, threshold: usize) -> Result<String, String> {
    let (short, long): (Vec<String>, Vec<String>) = items
        .into_iter()
        .partition(|s| s.len() < threshold);

    // TAINT SINK: Both partitions
    sinks::write_to_file("/tmp/short.txt", &short.join(","))?;
    sinks::write_to_file("/tmp/long.txt", &long.join(","))
}

/// Collect with try_fold
pub fn try_fold_sink(items: Vec<String>) -> Result<String, String> {
    let result = items.into_iter().try_fold(String::new(), |mut acc, item| {
        if item.contains("error") {
            return Err("Found error marker".to_string());
        }
        acc.push_str(&item);
        acc.push('|');
        Ok(acc)
    })?;

    // TAINT SINK: Try fold result
    sinks::execute_shell(&format!("echo {}", result))
}

/// Group by with tainted keys
pub fn group_by_first_char(items: Vec<String>) -> Result<String, String> {
    let mut groups: HashMap<char, Vec<String>> = HashMap::new();

    for item in items {
        if let Some(c) = item.chars().next() {
            // Group key derived from tainted data
            groups.entry(c).or_default().push(item);
        }
    }

    // TAINT SINK: Grouped data
    let output: String = groups
        .into_iter()
        .map(|(k, v)| format!("{}: {}", k, v.join(",")))
        .collect::<Vec<_>>()
        .join("\n");

    sinks::write_to_file("/tmp/grouped.txt", &output)
}

// ============================================================================
// ITERATOR CONSUMERS
// ============================================================================

/// Find with tainted predicate
pub fn find_and_sink(items: Vec<String>, pattern: String) -> Result<String, String> {
    // Pattern is tainted
    let found = items
        .into_iter()
        .find(|s| s.contains(&pattern));

    if let Some(item) = found {
        // TAINT SINK: Found item
        sinks::execute_query(&format!("INSERT INTO found VALUES ('{}')", item))
    } else {
        Ok("Not found".to_string())
    }
}

/// Position with sink
pub fn position_and_sink(items: Vec<String>, target: String) -> Result<String, String> {
    // Target is tainted
    let pos = items
        .iter()
        .position(|s| s == &target);

    // TAINT SINK: Position with tainted context
    let msg = format!("Target '{}' at position {:?}", target, pos);
    sinks::log_data("INFO", &msg);
    Ok(msg)
}

/// Any/all with tainted check
pub fn any_all_sink(items: Vec<String>, check: String) -> Result<String, String> {
    // Check string is tainted
    let has_any = items.iter().any(|s| s.contains(&check));
    let has_all = items.iter().all(|s| s.contains(&check));

    // TAINT SINK: Results with tainted context
    let result = format!("Check '{}': any={}, all={}", check, has_any, has_all);
    sinks::write_to_file("/tmp/any_all.txt", &result)
}

/// Max/min with tainted comparisons
pub fn max_min_sink(items: Vec<String>) -> Result<String, String> {
    let max = items.iter().max().cloned().unwrap_or_default();
    let min = items.iter().min().cloned().unwrap_or_default();

    // TAINT SINK: Max/min values
    sinks::execute_query(&format!(
        "INSERT INTO extremes (max, min) VALUES ('{}', '{}')",
        max, min
    ))
}
