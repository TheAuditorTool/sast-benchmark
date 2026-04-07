//! Data models for deepflow dataflow analysis.
//!
//! These models represent data that flows through complex pipelines,
//! async chains, and trait-based processing.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use std::sync::Arc;

// ============================================================================
// HTTP Request Models - TAINT SOURCES
// ============================================================================

/// Raw input from HTTP request - primary taint source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawInput {
    pub data: String,
    pub metadata: Option<HashMap<String, String>>,
}

/// Pipeline processing request - will flow through deep call chains
#[derive(Debug, Clone, Deserialize)]
pub struct PipelineRequest {
    pub input: String,
    pub stages: Vec<String>,
    pub config: Option<PipelineConfig>,
}

/// Configuration for pipeline processing
#[derive(Debug, Clone, Deserialize, Default)]
pub struct PipelineConfig {
    pub max_depth: Option<usize>,
    pub timeout_ms: Option<u64>,
    pub output_format: Option<String>,
}

/// Async workflow request - will flow through async/await chains
#[derive(Debug, Clone, Deserialize)]
pub struct AsyncWorkflowRequest {
    pub workflow_id: String,
    pub payload: String,
    pub callbacks: Option<Vec<String>>,
}

/// Query request for database operations
#[derive(Debug, Clone, Deserialize)]
pub struct QueryRequest {
    pub table: String,
    pub filter: Option<String>,
    pub columns: Option<Vec<String>>,
    pub order_by: Option<String>,
    pub limit: Option<i32>,
}

/// Command request for system operations
#[derive(Debug, Clone, Deserialize)]
pub struct CommandRequest {
    pub program: String,
    pub arguments: Vec<String>,
    pub working_directory: Option<String>,
    pub environment: Option<HashMap<String, String>>,
}

/// File operation request
#[derive(Debug, Clone, Deserialize)]
pub struct FileRequest {
    pub path: String,
    pub operation: FileOperation,
    pub content: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FileOperation {
    Read,
    Write,
    Append,
    Delete,
}

/// Network request for SSRF testing
#[derive(Debug, Clone, Deserialize)]
pub struct NetworkRequest {
    pub url: String,
    pub method: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<String>,
}

/// Batch processing request - multiple items flow together
#[derive(Debug, Clone, Deserialize)]
pub struct BatchRequest {
    pub items: Vec<BatchItem>,
    pub parallel: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchItem {
    pub id: String,
    pub data: String,
    pub priority: Option<i32>,
}

// ============================================================================
// Processing Stage Models - Data flows through these
// ============================================================================

/// Represents data at a processing stage
#[derive(Debug, Clone)]
pub struct StageData {
    pub content: String,
    pub stage_name: String,
    pub transformations: Vec<String>,
}

impl StageData {
    pub fn new(content: String, stage: &str) -> Self {
        Self {
            content,
            stage_name: stage.to_string(),
            transformations: vec![stage.to_string()],
        }
    }

    pub fn transform(mut self, stage: &str, new_content: String) -> Self {
        self.content = new_content;
        self.stage_name = stage.to_string();
        self.transformations.push(stage.to_string());
        self
    }
}

/// Context passed through async workflows
#[derive(Debug, Clone)]
pub struct WorkflowContext {
    pub workflow_id: String,
    pub current_step: String,
    pub data: String,
    pub results: HashMap<String, String>,
}

impl WorkflowContext {
    pub fn new(workflow_id: String, initial_data: String) -> Self {
        Self {
            workflow_id,
            current_step: "init".to_string(),
            data: initial_data,
            results: HashMap::new(),
        }
    }
}

/// Shared state that may contain tainted data
#[derive(Debug, Clone)]
pub struct SharedState {
    pub cache: HashMap<String, String>,
    pub pending_operations: Vec<PendingOp>,
}

#[derive(Debug, Clone)]
pub struct PendingOp {
    pub op_type: String,
    pub data: String,
}

/// Synchronized wrapper for shared data
pub type ArcState = Arc<tokio::sync::RwLock<SharedState>>;

// ============================================================================
// Response Models
// ============================================================================

/// Standard API response
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub metadata: Option<ResponseMetadata>,
}

#[derive(Debug, Serialize)]
pub struct ResponseMetadata {
    pub processing_time_ms: u64,
    pub stages_executed: Vec<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            metadata: None,
        }
    }

    pub fn success_with_metadata(data: T, metadata: ResponseMetadata) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            metadata: Some(metadata),
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message.to_string()),
            metadata: None,
        }
    }
}

// ============================================================================
// Trait Object Models - For polymorphic dataflow
// ============================================================================

/// A boxed processor that can be stored and invoked dynamically
pub type BoxedProcessor = Box<dyn DataProcessor + Send + Sync>;

/// Trait for data processors - enables polymorphic dataflow
pub trait DataProcessor: dyn_clone::DynClone {
    fn process(&self, input: &str) -> String;
    fn name(&self) -> &str;
}

dyn_clone::clone_trait_object!(DataProcessor);

/// Simple prefix processor
#[derive(Clone)]
pub struct PrefixProcessor {
    pub prefix: String,
}

impl DataProcessor for PrefixProcessor {
    fn process(&self, input: &str) -> String {
        format!("{}{}", self.prefix, input)
    }
    fn name(&self) -> &str {
        "prefix"
    }
}

/// Suffix processor
#[derive(Clone)]
pub struct SuffixProcessor {
    pub suffix: String,
}

impl DataProcessor for SuffixProcessor {
    fn process(&self, input: &str) -> String {
        format!("{}{}", input, self.suffix)
    }
    fn name(&self) -> &str {
        "suffix"
    }
}

/// Transform processor - applies arbitrary transformation
#[derive(Clone)]
pub struct TransformProcessor {
    pub transform_type: String,
}

impl DataProcessor for TransformProcessor {
    fn process(&self, input: &str) -> String {
        match self.transform_type.as_str() {
            "uppercase" => input.to_uppercase(),
            "lowercase" => input.to_lowercase(),
            "reverse" => input.chars().rev().collect(),
            "base64" => base64_encode(input),
            _ => input.to_string(),
        }
    }
    fn name(&self) -> &str {
        "transform"
    }
}

/// Base64 encode helper
fn base64_encode(input: &str) -> String {
    use std::io::Write;
    let mut buf = Vec::new();
    {
        let mut encoder = Base64Encoder::new(&mut buf);
        encoder.write_all(input.as_bytes()).unwrap();
    }
    String::from_utf8(buf).unwrap_or_default()
}

/// Simple base64 encoder (minimal implementation for demo)
struct Base64Encoder<W: Write> {
    writer: W,
}

impl<W: Write> Base64Encoder<W> {
    fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<W: Write> Write for Base64Encoder<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        for chunk in buf.chunks(3) {
            let b0 = chunk[0] as usize;
            let b1 = chunk.get(1).copied().unwrap_or(0) as usize;
            let b2 = chunk.get(2).copied().unwrap_or(0) as usize;

            let c0 = ALPHABET[b0 >> 2];
            let c1 = ALPHABET[((b0 & 0x03) << 4) | (b1 >> 4)];
            let c2 = if chunk.len() > 1 {
                ALPHABET[((b1 & 0x0f) << 2) | (b2 >> 6)]
            } else {
                b'='
            };
            let c3 = if chunk.len() > 2 {
                ALPHABET[b2 & 0x3f]
            } else {
                b'='
            };

            self.writer.write_all(&[c0, c1, c2, c3])?;
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}
