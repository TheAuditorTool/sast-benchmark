//! Trait-based polymorphic dataflow - data flows through trait objects.
//!
//! This module demonstrates taint propagation through:
//! - Dynamic dispatch (dyn Trait)
//! - Generic functions with trait bounds
//! - Trait object chains
//! - Associated types
//! - Default implementations

use crate::sinks;
use std::collections::HashMap;

// ============================================================================
// TRAIT DEFINITIONS
// ============================================================================

/// Base processor trait - all implementations process tainted data
pub trait Processor: Send + Sync {
    fn process(&self, input: &str) -> String;
    fn name(&self) -> &str;
}

/// Transformer trait with associated type
pub trait Transformer {
    type Output;
    fn transform(&self, input: String) -> Self::Output;
}

/// Sink trait - implementations write to dangerous sinks
pub trait SinkWriter {
    fn write(&self, data: &str) -> Result<String, String>;
}

/// Chainable processor
pub trait ChainableProcessor: Processor {
    fn chain(&self, next: Box<dyn Processor>) -> ProcessorChain;
}

/// Async processor trait (placeholder - would need async_trait crate for actual impl)
pub trait AsyncProcessor: Send + Sync {
    /// Process input asynchronously - returns future in real implementation
    fn process_async(&self, input: String) -> Result<String, String>;
}

// ============================================================================
// PROCESSOR IMPLEMENTATIONS
// ============================================================================

/// Uppercase processor
pub struct UppercaseProcessor;

impl Processor for UppercaseProcessor {
    fn process(&self, input: &str) -> String {
        input.to_uppercase()
    }
    fn name(&self) -> &str {
        "uppercase"
    }
}

/// Prefix processor with state
pub struct PrefixProcessor {
    prefix: String,
}

impl PrefixProcessor {
    pub fn new(prefix: String) -> Self {
        Self { prefix }
    }
}

impl Processor for PrefixProcessor {
    fn process(&self, input: &str) -> String {
        // Prefix might also be tainted
        format!("{}{}", self.prefix, input)
    }
    fn name(&self) -> &str {
        "prefix"
    }
}

/// Suffix processor
pub struct SuffixProcessor {
    suffix: String,
}

impl SuffixProcessor {
    pub fn new(suffix: String) -> Self {
        Self { suffix }
    }
}

impl Processor for SuffixProcessor {
    fn process(&self, input: &str) -> String {
        format!("{}{}", input, self.suffix)
    }
    fn name(&self) -> &str {
        "suffix"
    }
}

/// Encoding processor
pub struct Base64Processor;

impl Processor for Base64Processor {
    fn process(&self, input: &str) -> String {
        // Simple base64-like encoding
        input.bytes().map(|b| format!("{:02x}", b)).collect()
    }
    fn name(&self) -> &str {
        "base64"
    }
}

/// Dangerous processor that writes to sink
pub struct DangerousSinkProcessor {
    sink_type: String,
}

impl DangerousSinkProcessor {
    pub fn new(sink_type: &str) -> Self {
        Self {
            sink_type: sink_type.to_string(),
        }
    }
}

impl Processor for DangerousSinkProcessor {
    fn process(&self, input: &str) -> String {
        // TAINT SINK: Processor that writes to dangerous sink
        match self.sink_type.as_str() {
            "file" => {
                let _ = sinks::write_to_file("/tmp/trait_sink.txt", input);
            }
            "command" => {
                let _ = sinks::execute_shell(&format!("echo {}", input));
            }
            "sql" => {
                let _ = sinks::execute_query(&format!("INSERT INTO log VALUES ('{}')", input));
            }
            "network" => {
                let _ = sinks::fetch_url(&format!("http://api/log?data={}", input));
            }
            _ => {}
        }
        input.to_string()
    }
    fn name(&self) -> &str {
        "dangerous_sink"
    }
}

// ============================================================================
// PROCESSOR CHAIN - Multiple processors in sequence
// ============================================================================

/// Chain of processors - data flows through each
pub struct ProcessorChain {
    processors: Vec<Box<dyn Processor>>,
}

impl ProcessorChain {
    pub fn new() -> Self {
        Self {
            processors: Vec::new(),
        }
    }

    pub fn add(mut self, processor: Box<dyn Processor>) -> Self {
        self.processors.push(processor);
        self
    }

    /// Execute chain - tainted data flows through all processors
    pub fn execute(&self, input: String) -> String {
        let mut data = input;
        for processor in &self.processors {
            // Data flows through each processor, maintaining taint
            data = processor.process(&data);
        }
        data
    }

    /// Execute and write to sink
    pub fn execute_to_sink(&self, input: String) -> Result<String, String> {
        let result = self.execute(input);
        // TAINT SINK: Chain result flows to file
        sinks::write_to_file("/tmp/chain_result.txt", &result)
    }
}

impl Default for ProcessorChain {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// GENERIC FUNCTIONS WITH TRAIT BOUNDS
// ============================================================================

/// Process with any Processor implementation
pub fn process_with<P: Processor + ?Sized>(processor: &P, input: String) -> String {
    // Generic - taint flows through regardless of concrete type
    processor.process(&input)
}

/// Process and sink with generic processor
pub fn process_and_sink<P: Processor>(processor: &P, input: String) -> Result<String, String> {
    let result = processor.process(&input);
    // TAINT SINK: Generic result flows to sink
    sinks::execute_shell(&format!("echo {}", result))
}

/// Chain two processors generically
pub fn chain_process<P1, P2>(p1: &P1, p2: &P2, input: String) -> String
where
    P1: Processor,
    P2: Processor,
{
    let intermediate = p1.process(&input);
    // Intermediate data is still tainted
    p2.process(&intermediate)
}

/// Multiple trait bounds
pub fn process_multiple<P>(processor: &P, input: String) -> String
where
    P: Processor + Send + Sync,
{
    processor.process(&input)
}

// ============================================================================
// TRAIT OBJECTS (DYNAMIC DISPATCH)
// ============================================================================

/// Process through trait object
pub fn process_dyn(processor: &dyn Processor, input: String) -> String {
    // Dynamic dispatch - taint flows through trait object
    processor.process(&input)
}

/// Process through boxed trait object
pub fn process_boxed(processor: Box<dyn Processor>, input: String) -> String {
    processor.process(&input)
}

/// Vector of trait objects
pub fn process_all(processors: &[Box<dyn Processor>], input: String) -> Vec<String> {
    processors
        .iter()
        .map(|p| p.process(&input))
        .collect()
}

/// Select processor at runtime
pub fn select_and_process(
    processors: &HashMap<String, Box<dyn Processor>>,
    processor_name: &str,
    input: String,
) -> Result<String, String> {
    let processor = processors
        .get(processor_name)
        .ok_or_else(|| format!("Processor {} not found", processor_name))?;

    let result = processor.process(&input);

    // TAINT SINK: Dynamically selected processor result
    sinks::write_to_file(&format!("/tmp/{}.out", processor_name), &result)
}

// ============================================================================
// TRANSFORMER IMPLEMENTATIONS (ASSOCIATED TYPES)
// ============================================================================

/// String to String transformer
pub struct StringTransformer {
    transform_fn: fn(String) -> String,
}

impl StringTransformer {
    pub fn new(f: fn(String) -> String) -> Self {
        Self { transform_fn: f }
    }
}

impl Transformer for StringTransformer {
    type Output = String;

    fn transform(&self, input: String) -> Self::Output {
        (self.transform_fn)(input)
    }
}

/// String to Vec transformer
pub struct SplitTransformer {
    delimiter: String,
}

impl SplitTransformer {
    pub fn new(delimiter: &str) -> Self {
        Self {
            delimiter: delimiter.to_string(),
        }
    }
}

impl Transformer for SplitTransformer {
    type Output = Vec<String>;

    fn transform(&self, input: String) -> Self::Output {
        // Split maintains taint in each part
        input.split(&self.delimiter).map(String::from).collect()
    }
}

/// Use transformer generically
pub fn transform_and_sink<T: Transformer<Output = String>>(
    transformer: &T,
    input: String,
) -> Result<String, String> {
    let result = transformer.transform(input);
    // TAINT SINK: Transformed data
    sinks::execute_query(&format!("INSERT INTO transformed VALUES ('{}')", result))
}

// ============================================================================
// SINK WRITER IMPLEMENTATIONS
// ============================================================================

/// File sink
pub struct FileSink {
    path: String,
}

impl FileSink {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }
}

impl SinkWriter for FileSink {
    fn write(&self, data: &str) -> Result<String, String> {
        // TAINT SINK: Path might be tainted, data is tainted
        sinks::write_to_file(&self.path, data)
    }
}

/// Command sink
pub struct CommandSink {
    base_command: String,
}

impl CommandSink {
    pub fn new(base: &str) -> Self {
        Self {
            base_command: base.to_string(),
        }
    }
}

impl SinkWriter for CommandSink {
    fn write(&self, data: &str) -> Result<String, String> {
        // TAINT SINK: Command execution with user-controlled data
        sinks::execute_shell(&format!("{} {}", self.base_command, data))
    }
}

/// Database sink
pub struct DatabaseSink {
    table: String,
}

impl DatabaseSink {
    pub fn new(table: &str) -> Self {
        Self {
            table: table.to_string(),
        }
    }
}

impl SinkWriter for DatabaseSink {
    fn write(&self, data: &str) -> Result<String, String> {
        // TAINT SINK: SQL query with user-controlled data
        sinks::execute_query(&format!(
            "INSERT INTO {} (data) VALUES ('{}')",
            self.table, data
        ))
    }
}

/// Network sink
pub struct NetworkSink {
    base_url: String,
}

impl NetworkSink {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }
}

impl SinkWriter for NetworkSink {
    fn write(&self, data: &str) -> Result<String, String> {
        // TAINT SINK: SSRF
        sinks::fetch_url(&format!("{}/{}", self.base_url, data))
    }
}

/// Process and write to generic sink
pub fn process_to_sink<S: SinkWriter>(
    processor: &dyn Processor,
    sink: &S,
    input: String,
) -> Result<String, String> {
    let result = processor.process(&input);
    // Result flows to sink through trait
    sink.write(&result)
}

// ============================================================================
// DEFAULT TRAIT IMPLEMENTATIONS
// ============================================================================

/// Trait with default implementation
pub trait ProcessorWithDefaults {
    fn process(&self, input: &str) -> String;

    /// Default pre-processing - taint flows through
    fn preprocess(&self, input: &str) -> String {
        format!("PRE[{}]", input)
    }

    /// Default post-processing
    fn postprocess(&self, output: &str) -> String {
        format!("POST[{}]", output)
    }

    /// Full processing with defaults
    fn full_process(&self, input: &str) -> String {
        let pre = self.preprocess(input);
        let processed = self.process(&pre);
        self.postprocess(&processed)
    }
}

/// Minimal implementation - uses defaults
pub struct MinimalProcessor;

impl ProcessorWithDefaults for MinimalProcessor {
    fn process(&self, input: &str) -> String {
        input.to_uppercase()
    }
}

/// Custom implementation - overrides defaults
pub struct CustomProcessor;

impl ProcessorWithDefaults for CustomProcessor {
    fn process(&self, input: &str) -> String {
        format!("CUSTOM[{}]", input)
    }

    fn preprocess(&self, input: &str) -> String {
        format!("CUSTOM_PRE[{}]", input)
    }
}

/// Process using defaults
pub fn process_with_defaults<P: ProcessorWithDefaults>(processor: &P, input: String) -> String {
    // Taint flows through default implementations
    processor.full_process(&input)
}

// ============================================================================
// TRAIT OBJECT FACTORY
// ============================================================================

/// Factory for creating processors dynamically
pub fn create_processor(name: &str, config: Option<String>) -> Box<dyn Processor> {
    match name {
        "uppercase" => Box::new(UppercaseProcessor),
        "prefix" => Box::new(PrefixProcessor::new(config.unwrap_or_default())),
        "suffix" => Box::new(SuffixProcessor::new(config.unwrap_or_default())),
        "base64" => Box::new(Base64Processor),
        "file_sink" => Box::new(DangerousSinkProcessor::new("file")),
        "cmd_sink" => Box::new(DangerousSinkProcessor::new("command")),
        "sql_sink" => Box::new(DangerousSinkProcessor::new("sql")),
        _ => Box::new(UppercaseProcessor),
    }
}

/// Build processor chain from names
pub fn build_chain(processor_names: Vec<String>, configs: Vec<Option<String>>) -> ProcessorChain {
    let mut chain = ProcessorChain::new();
    for (name, config) in processor_names.iter().zip(configs.iter()) {
        chain = chain.add(create_processor(name, config.clone()));
    }
    chain
}
