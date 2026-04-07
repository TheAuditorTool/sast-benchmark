//! Closure capture dataflow - data captured by closures.
//!
//! This module demonstrates taint propagation through:
//! - Closure captures (move, borrow, ref)
//! - Higher-order functions
//! - Callback patterns
//! - Stored closures
//! - Closure factories

use crate::sinks;
use std::collections::HashMap;

// ============================================================================
// CLOSURE CAPTURE PATTERNS
// ============================================================================

/// Closure captures tainted data by reference
pub fn capture_by_ref(input: String) -> Result<String, String> {
    // Closure borrows input
    let processor = || {
        // input is borrowed - still tainted
        format!("REF[{}]", input)
    };

    let result = processor();

    // TAINT SINK: Closure result contains tainted data
    sinks::write_to_file("/tmp/ref_capture.txt", &result)
}

/// Closure captures tainted data by move
pub fn capture_by_move(input: String) -> Result<String, String> {
    // Closure takes ownership
    let processor = move || {
        // input is moved - still tainted
        format!("MOVE[{}]", input)
    };

    let result = processor();

    // TAINT SINK: Moved data flows to sink
    sinks::execute_shell(&format!("echo {}", result))
}

/// Multiple closures capturing same data
pub fn multiple_captures(input: String) -> Result<String, String> {
    let input_clone = input.clone();

    // First closure
    let closure1 = || format!("C1[{}]", input);

    // Second closure with clone
    let closure2 = move || format!("C2[{}]", input_clone);

    let r1 = closure1();
    let r2 = closure2();

    // TAINT SINK: Both results tainted
    let combined = format!("{}-{}", r1, r2);
    sinks::execute_query(&format!("INSERT INTO closures VALUES ('{}')", combined))
}

/// Nested closure captures
pub fn nested_captures(input: String) -> Result<String, String> {
    let outer = || {
        let inner = || {
            // Inner closure accesses outer's captured data
            format!("INNER[{}]", input)
        };
        format!("OUTER[{}]", inner())
    };

    let result = outer();

    // TAINT SINK: Nested closure result
    sinks::fetch_url(&format!("http://api/nested?data={}", result))
}

// ============================================================================
// HIGHER-ORDER FUNCTIONS
// ============================================================================

/// Apply function to tainted data
pub fn apply_fn<F>(input: String, f: F) -> String
where
    F: FnOnce(String) -> String,
{
    // Tainted data passed to function
    f(input)
}

/// Apply and sink
pub fn apply_and_sink<F>(input: String, f: F) -> Result<String, String>
where
    F: FnOnce(String) -> String,
{
    let result = f(input);
    // TAINT SINK: Applied function result
    sinks::write_to_file("/tmp/applied.txt", &result)
}

/// Map over tainted data
pub fn map_tainted<F>(input: String, f: F) -> Result<String, String>
where
    F: Fn(char) -> char,
{
    let result: String = input.chars().map(f).collect();
    // TAINT SINK: Mapped data still tainted
    sinks::execute_shell(&format!("echo {}", result))
}

/// Fold with tainted data
pub fn fold_tainted<F>(items: Vec<String>, init: String, f: F) -> Result<String, String>
where
    F: Fn(String, String) -> String,
{
    let result = items.into_iter().fold(init, f);
    // TAINT SINK: Folded result includes all tainted items
    sinks::write_to_file("/tmp/folded.txt", &result)
}

/// Filter and process
pub fn filter_process<F>(items: Vec<String>, predicate: F) -> Result<String, String>
where
    F: Fn(&String) -> bool,
{
    let filtered: Vec<String> = items.into_iter().filter(predicate).collect();
    // TAINT SINK: Filtered items still tainted
    let joined = filtered.join(",");
    sinks::execute_query(&format!("INSERT INTO filtered VALUES ('{}')", joined))
}

// ============================================================================
// CALLBACK PATTERNS
// ============================================================================

/// Callback with tainted data
pub fn with_callback<F>(input: String, callback: F) -> Result<String, String>
where
    F: FnOnce(&str) -> Result<String, String>,
{
    let processed = format!("CB[{}]", input);
    // Callback receives tainted data
    callback(&processed)
}

/// Deferred callback
pub fn defer_callback<F>(input: String, callback: F)
where
    F: FnOnce(String) + Send + 'static,
{
    // Callback will be called with tainted data
    std::thread::spawn(move || {
        callback(input);
    });
}

/// Success/error callbacks
pub fn with_result_callbacks<S, E>(
    input: String,
    on_success: S,
    on_error: E,
) -> Result<String, String>
where
    S: FnOnce(String) -> String,
    E: FnOnce(String) -> String,
{
    if input.is_empty() {
        // Error callback receives tainted input
        let err_result = on_error("Empty input".to_string());
        // TAINT SINK: Error path
        return sinks::write_to_file("/tmp/error.txt", &err_result);
    }

    // Success callback receives tainted input
    let success_result = on_success(input);
    // TAINT SINK: Success path
    sinks::write_to_file("/tmp/success.txt", &success_result)
}

/// Builder with callbacks
pub struct CallbackBuilder {
    data: String,
    transforms: Vec<Box<dyn Fn(String) -> String>>,
}

impl CallbackBuilder {
    pub fn new(input: String) -> Self {
        Self {
            data: input,
            transforms: Vec::new(),
        }
    }

    pub fn transform<F: Fn(String) -> String + 'static>(mut self, f: F) -> Self {
        self.transforms.push(Box::new(f));
        self
    }

    pub fn build(self) -> Result<String, String> {
        let mut result = self.data;
        for transform in self.transforms {
            // Each transform receives tainted data
            result = transform(result);
        }
        // TAINT SINK: Final result after all transforms
        sinks::execute_shell(&format!("echo {}", result))
    }
}

// ============================================================================
// STORED CLOSURES
// ============================================================================

/// Container for stored closures
pub struct ClosureStore {
    processors: HashMap<String, Box<dyn Fn(&str) -> String>>,
}

impl ClosureStore {
    pub fn new() -> Self {
        Self {
            processors: HashMap::new(),
        }
    }

    /// Register closure that captures config
    pub fn register_with_config(&mut self, name: &str, config: String) {
        // Config might be tainted
        self.processors.insert(
            name.to_string(),
            Box::new(move |input| {
                // Both config and input can be tainted
                format!("{}[{}]", config, input)
            }),
        );
    }

    /// Process with stored closure
    pub fn process(&self, name: &str, input: &str) -> Option<String> {
        self.processors.get(name).map(|f| f(input))
    }

    /// Process and sink
    pub fn process_to_sink(&self, name: &str, input: &str) -> Result<String, String> {
        let result = self
            .process(name, input)
            .ok_or_else(|| "Processor not found".to_string())?;

        // TAINT SINK: Stored closure result
        sinks::write_to_file(&format!("/tmp/{}.txt", name), &result)
    }
}

impl Default for ClosureStore {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// CLOSURE FACTORIES
// ============================================================================

/// Create closure that captures tainted prefix
pub fn make_prefixer(prefix: String) -> impl Fn(&str) -> String {
    // Prefix is captured (might be tainted)
    move |input| format!("{}{}", prefix, input)
}

/// Create closure that writes to sink
pub fn make_sink_writer(path: String) -> impl Fn(&str) -> Result<String, String> {
    // Path captured (might be tainted)
    move |data| {
        // TAINT SINK: Path from closure capture
        sinks::write_to_file(&path, data)
    }
}

/// Create closure chain
pub fn make_chain(
    transforms: Vec<String>,
) -> impl Fn(String) -> String {
    move |mut input| {
        for transform in &transforms {
            // Transform names might be tainted
            input = match transform.as_str() {
                "upper" => input.to_uppercase(),
                "lower" => input.to_lowercase(),
                "reverse" => input.chars().rev().collect(),
                _ => format!("[{}]{}", transform, input),
            };
        }
        input
    }
}

/// Create conditional closure
pub fn make_conditional(
    condition: String,
    if_true: String,
    if_false: String,
) -> impl Fn(&str) -> String {
    move |input| {
        // Condition and branches might be tainted
        if input.contains(&condition) {
            format!("{}{}", if_true, input)
        } else {
            format!("{}{}", if_false, input)
        }
    }
}

// ============================================================================
// FUNCTION TRAIT OBJECTS
// ============================================================================

/// Process through Fn trait object
pub fn process_fn_ref(processor: &dyn Fn(&str) -> String, input: &str) -> String {
    processor(input)
}

/// Process through FnMut trait object
pub fn process_fn_mut(processor: &mut dyn FnMut(&str) -> String, input: &str) -> String {
    processor(input)
}

/// Process through FnOnce (boxed)
pub fn process_fn_once(processor: Box<dyn FnOnce(String) -> String>, input: String) -> String {
    processor(input)
}

/// Chain multiple Fn trait objects
pub fn chain_fn_refs(
    processors: Vec<&dyn Fn(&str) -> String>,
    input: String,
) -> Result<String, String> {
    let mut data = input;
    for processor in processors {
        // Data flows through each, maintaining taint
        data = processor(&data);
    }
    // TAINT SINK: Chain result
    sinks::execute_query(&format!("INSERT INTO chain VALUES ('{}')", data))
}

// ============================================================================
// CLOSURE WITH SIDE EFFECTS
// ============================================================================

/// Closure that writes to sink as side effect
pub fn closure_with_side_effect(input: String) -> Result<String, String> {
    let mut log = Vec::new();

    let processor = |data: &str| -> String {
        // TAINT SINK: Side effect writes tainted data
        let _ = sinks::log_data("DEBUG", data);
        format!("PROCESSED[{}]", data)
    };

    log.push(processor(&input));

    // TAINT SINK: Log contains tainted data
    sinks::write_to_file("/tmp/side_effect_log.txt", &log.join("\n"))
}

/// Closure that accumulates state
pub fn accumulating_closure(items: Vec<String>) -> Result<String, String> {
    let mut accumulated = String::new();

    let mut accumulator = |item: &str| {
        // Each item is tainted
        accumulated.push_str(item);
        accumulated.push_str(";");
    };

    for item in &items {
        accumulator(item);
    }

    // TAINT SINK: Accumulated data
    sinks::execute_shell(&format!("echo {}", accumulated))
}

// ============================================================================
// CLOSURE IN STRUCT
// ============================================================================

/// Struct holding closure
pub struct ProcessorHolder {
    name: String,
    processor: Box<dyn Fn(String) -> String>,
}

impl ProcessorHolder {
    pub fn new<F>(name: &str, processor: F) -> Self
    where
        F: Fn(String) -> String + 'static,
    {
        Self {
            name: name.to_string(),
            processor: Box::new(processor),
        }
    }

    pub fn process(&self, input: String) -> String {
        (self.processor)(input)
    }

    pub fn process_to_sink(&self, input: String) -> Result<String, String> {
        let result = self.process(input);
        // TAINT SINK: Holder's processor result
        sinks::write_to_file(&format!("/tmp/{}.txt", self.name), &result)
    }
}

/// Create holder with captured config
pub fn create_holder(name: &str, config: String) -> ProcessorHolder {
    // Config is captured in closure
    ProcessorHolder::new(name, move |input| {
        format!("{}[{}]", config, input)
    })
}

// ============================================================================
// ASYNC CLOSURES
// ============================================================================

/// Async closure pattern (using regular async fn as closure isn't stable)
pub async fn async_closure_pattern(input: String) -> Result<String, String> {
    let captured = input.clone();

    // Simulate async closure with async block
    let future = async move {
        // Captured data is tainted
        format!("ASYNC[{}]", captured)
    };

    let result = future.await;

    // TAINT SINK: Async closure result
    sinks::fetch_url(&format!("http://api/async?data={}", result))
}
