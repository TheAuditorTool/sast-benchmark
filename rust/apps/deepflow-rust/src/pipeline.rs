//! Deep dataflow pipeline - data flows through many function calls before sinks.
//!
//! This module demonstrates call chains where tainted data passes through
//! 5+ levels of function calls before reaching a dangerous sink.

use crate::models::{PipelineConfig, StageData};
use crate::sinks;
use std::collections::HashMap;

// ============================================================================
// DEEP CALL CHAIN: 7 levels from source to sink
// ============================================================================

/// Level 1: Entry point - receives tainted input
/// TAINT FLOW: input -> process_stage_1
pub fn execute_pipeline(input: String, stages: Vec<String>) -> Result<String, String> {
    // Data enters here from HTTP handler (tainted)
    let stage_data = StageData::new(input, "entry");
    process_stage_1(stage_data, stages)
}

/// Level 2: First processing stage
fn process_stage_1(data: StageData, stages: Vec<String>) -> Result<String, String> {
    let processed = format!("[stage1]{}", data.content);
    let new_data = data.transform("stage1", processed);
    process_stage_2(new_data, stages)
}

/// Level 3: Second processing stage
fn process_stage_2(data: StageData, stages: Vec<String>) -> Result<String, String> {
    let processed = data.content.replace("stage1", "STAGE1");
    let new_data = data.transform("stage2", processed);
    process_stage_3(new_data, stages)
}

/// Level 4: Third processing stage - branching logic
fn process_stage_3(data: StageData, stages: Vec<String>) -> Result<String, String> {
    let processed = if stages.contains(&"uppercase".to_string()) {
        data.content.to_uppercase()
    } else {
        data.content.clone()
    };
    let new_data = data.transform("stage3", processed);
    process_stage_4(new_data, stages)
}

/// Level 5: Fourth processing stage
fn process_stage_4(data: StageData, stages: Vec<String>) -> Result<String, String> {
    let processed = format!("{{processed: \"{}\"}}", data.content);
    let new_data = data.transform("stage4", processed);
    process_stage_5(new_data, stages)
}

/// Level 6: Fifth processing stage
fn process_stage_5(data: StageData, stages: Vec<String>) -> Result<String, String> {
    let processed = data.content.trim().to_string();
    let new_data = data.transform("stage5", processed);
    process_stage_6(new_data, stages)
}

/// Level 7: Final stage - reaches SINK
fn process_stage_6(data: StageData, stages: Vec<String>) -> Result<String, String> {
    // TAINT SINK: After 6 levels, data reaches command execution
    if stages.contains(&"execute".to_string()) {
        // VULNERABLE: Tainted data flows to command execution after deep chain
        return sinks::execute_shell(&data.content);
    }

    // TAINT SINK: Or flows to file write
    if stages.contains(&"save".to_string()) {
        let path = format!("/tmp/pipeline_{}.out", data.stage_name);
        sinks::write_to_file(&path, &data.content)?;
    }

    Ok(data.content)
}

// ============================================================================
// DEEP CALL CHAIN WITH STRUCT FIELD PROPAGATION
// ============================================================================

/// Configuration that carries tainted data through processing
#[derive(Clone)]
pub struct ProcessingContext {
    pub input_data: String,
    pub intermediate: String,
    pub config: HashMap<String, String>,
}

/// Level 1: Create context with tainted data
pub fn process_with_context(input: String, config: PipelineConfig) -> Result<String, String> {
    let ctx = ProcessingContext {
        input_data: input.clone(),
        intermediate: input,
        config: HashMap::new(),
    };
    context_stage_1(ctx, config)
}

/// Level 2: Process with context
fn context_stage_1(mut ctx: ProcessingContext, config: PipelineConfig) -> Result<String, String> {
    ctx.intermediate = format!("CTX1[{}]", ctx.intermediate);
    context_stage_2(ctx, config)
}

/// Level 3: More processing
fn context_stage_2(mut ctx: ProcessingContext, config: PipelineConfig) -> Result<String, String> {
    ctx.intermediate = ctx.intermediate.replace("[", "(").replace("]", ")");
    context_stage_3(ctx, config)
}

/// Level 4: Branch based on config
fn context_stage_3(mut ctx: ProcessingContext, config: PipelineConfig) -> Result<String, String> {
    if let Some(format) = &config.output_format {
        ctx.config.insert("format".to_string(), format.clone());
    }
    ctx.intermediate = format!("CTX3{{{}}}", ctx.intermediate);
    context_stage_4(ctx, config)
}

/// Level 5: Final processing before sink
fn context_stage_4(ctx: ProcessingContext, config: PipelineConfig) -> Result<String, String> {
    let output = format!("FINAL:{}", ctx.intermediate);

    // TAINT SINK: Tainted data from struct field flows to database
    if let Some(format) = ctx.config.get("format") {
        if format == "sql" {
            // VULNERABLE: ctx.input_data is tainted
            let query = format!(
                "INSERT INTO results (data) VALUES ('{}')",
                ctx.input_data
            );
            return sinks::execute_query(&query);
        }
    }

    // TAINT SINK: Tainted output flows to file
    if let Some(depth) = config.max_depth {
        if depth > 0 {
            let path = format!("/tmp/context_{}.dat", depth);
            // VULNERABLE: output contains tainted data
            sinks::write_to_file(&path, &output)?;
        }
    }

    Ok(output)
}

// ============================================================================
// RECURSIVE DATAFLOW - Data flows through recursive calls
// ============================================================================

/// Recursive processing - data flows through N recursive calls
pub fn recursive_process(input: String, depth: usize) -> Result<String, String> {
    recursive_level(input, depth, 0)
}

fn recursive_level(data: String, max_depth: usize, current: usize) -> Result<String, String> {
    if current >= max_depth {
        // TAINT SINK: At recursion bottom, data flows to network
        // VULNERABLE: Tainted data used in URL
        return sinks::fetch_url(&format!("http://internal/{}", data));
    }

    let processed = format!("R{}[{}]", current, data);
    recursive_level(processed, max_depth, current + 1)
}

// ============================================================================
// MUTUAL RECURSION - Data flows between two functions
// ============================================================================

/// Entry point for mutual recursion
pub fn mutual_process(input: String, iterations: usize) -> Result<String, String> {
    mutual_a(input, iterations)
}

fn mutual_a(data: String, remaining: usize) -> Result<String, String> {
    if remaining == 0 {
        // TAINT SINK: Data reaches sink after mutual recursion
        return sinks::execute_shell(&format!("echo {}", data));
    }
    let processed = format!("A({})", data);
    mutual_b(processed, remaining - 1)
}

fn mutual_b(data: String, remaining: usize) -> Result<String, String> {
    if remaining == 0 {
        // TAINT SINK: Alternative exit point
        return sinks::write_to_file("/tmp/mutual.out", &data);
    }
    let processed = format!("B({})", data);
    mutual_a(processed, remaining - 1)
}

// ============================================================================
// BUILDER PATTERN - Data flows through method chain
// ============================================================================

/// Builder that accumulates tainted data
pub struct PipelineBuilder {
    stages: Vec<String>,
    data: String,
    transforms: Vec<Box<dyn Fn(String) -> String + Send>>,
}

impl PipelineBuilder {
    /// Create with tainted input
    pub fn new(input: String) -> Self {
        Self {
            stages: vec!["init".to_string()],
            data: input,
            transforms: Vec::new(),
        }
    }

    /// Add transform - data still tainted
    pub fn add_transform<F>(mut self, name: &str, f: F) -> Self
    where
        F: Fn(String) -> String + Send + 'static,
    {
        self.stages.push(name.to_string());
        self.transforms.push(Box::new(f));
        self
    }

    /// Apply uppercase transform
    pub fn uppercase(self) -> Self {
        self.add_transform("uppercase", |s| s.to_uppercase())
    }

    /// Apply prefix
    pub fn prefix(self, p: String) -> Self {
        self.add_transform("prefix", move |s| format!("{}{}", p, s))
    }

    /// Apply suffix
    pub fn suffix(self, s: String) -> Self {
        let suffix = s;
        self.add_transform("suffix", move |data| format!("{}{}", data, suffix))
    }

    /// Execute pipeline - data flows through all transforms then to sink
    pub fn execute(mut self) -> Result<String, String> {
        // Apply all transforms
        for transform in &self.transforms {
            self.data = transform(self.data);
        }

        // TAINT SINK: Final data still tainted, flows to file
        let output_path = format!("/tmp/builder_{}.out", self.stages.len());
        sinks::write_to_file(&output_path, &self.data)?;

        Ok(self.data)
    }

    /// Execute to command - DANGEROUS
    pub fn execute_as_command(mut self) -> Result<String, String> {
        for transform in &self.transforms {
            self.data = transform(self.data);
        }

        // TAINT SINK: Tainted data flows to command execution
        sinks::execute_shell(&self.data)
    }
}

// ============================================================================
// DATA SPLITTING AND MERGING
// ============================================================================

/// Split tainted data, process separately, merge back
pub fn split_and_merge(input: String) -> Result<String, String> {
    let parts: Vec<&str> = input.split(',').collect();

    let processed_parts: Vec<String> = parts
        .iter()
        .enumerate()
        .map(|(i, part)| process_part(part, i))
        .collect();

    let merged = processed_parts.join(";");

    // TAINT SINK: Merged data still contains taint from all parts
    let query = format!("SELECT * FROM data WHERE id IN ({})", merged);
    sinks::execute_query(&query)
}

fn process_part(part: &str, index: usize) -> String {
    format!("{}:{}", index, part.trim())
}

/// Fan-out processing - one input spawns multiple tainted flows
pub fn fan_out_process(input: String) -> Result<Vec<String>, String> {
    let mut results = Vec::new();

    // Flow 1: To file sink
    let flow1 = format!("flow1:{}", input);
    sinks::write_to_file("/tmp/flow1.out", &flow1)?;
    results.push(flow1);

    // Flow 2: To command sink
    let flow2 = format!("echo flow2:{}", input);
    let output = sinks::execute_shell(&flow2)?;
    results.push(output);

    // Flow 3: To network sink
    let flow3 = format!("http://api/data/{}", input);
    let response = sinks::fetch_url(&flow3)?;
    results.push(response);

    Ok(results)
}

// ============================================================================
// CONDITIONAL FLOW - Different paths to same sink
// ============================================================================

/// Data can reach sink through multiple conditional paths
pub fn conditional_flow(input: String, mode: &str) -> Result<String, String> {
    let processed = match mode {
        "direct" => direct_path(input),
        "transform" => transform_path(input),
        "complex" => complex_path(input),
        _ => input,
    };

    // All paths converge here - data still tainted regardless of path
    finalize_output(processed)
}

fn direct_path(data: String) -> String {
    data
}

fn transform_path(data: String) -> String {
    data.to_uppercase()
}

fn complex_path(data: String) -> String {
    let step1 = format!("[{}]", data);
    let step2 = step1.replace("[", "{").replace("]", "}");
    let step3 = format!("COMPLEX({})", step2);
    step3
}

fn finalize_output(data: String) -> Result<String, String> {
    // TAINT SINK: All conditional paths lead here
    sinks::execute_query(&format!("INSERT INTO logs (data) VALUES ('{}')", data))
}

// ============================================================================
// LOOP-BASED DATAFLOW
// ============================================================================

/// Data accumulates taint through loop iterations
pub fn loop_accumulate(initial: String, iterations: usize) -> Result<String, String> {
    let mut data = initial;

    for i in 0..iterations {
        data = format!("iter{}[{}]", i, data);
        // Each iteration maintains taint
    }

    // TAINT SINK: After loop, data still tainted
    sinks::write_to_file("/tmp/loop_result.out", &data)
}

/// While loop with tainted condition
pub fn while_process(input: String, max_len: usize) -> Result<String, String> {
    let mut data = input;
    let mut counter = 0;

    while data.len() < max_len && counter < 100 {
        data = format!("W[{}]", data);
        counter += 1;
    }

    // TAINT SINK: Data expanded but still tainted
    sinks::execute_shell(&format!("echo {}", data))
}
