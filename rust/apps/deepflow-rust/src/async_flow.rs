//! Async dataflow patterns - data flows through async/await chains.
//!
//! This module demonstrates taint propagation through:
//! - Async function chains
//! - Futures and combinators
//! - Channels (mpsc, oneshot, broadcast)
//! - Spawned tasks
//! - Select/join patterns

use crate::models::WorkflowContext;
use crate::sinks;
use std::collections::HashMap;
use tokio::sync::{mpsc, oneshot, RwLock};
use std::sync::Arc;

// ============================================================================
// ASYNC CHAIN - Data flows through async/await
// ============================================================================

/// Entry point: tainted data enters async chain
pub async fn async_pipeline(input: String) -> Result<String, String> {
    // Data enters async world
    let step1 = async_step_1(input).await?;
    let step2 = async_step_2(step1).await?;
    let step3 = async_step_3(step2).await?;
    async_final_step(step3).await
}

async fn async_step_1(data: String) -> Result<String, String> {
    // Simulate async work
    tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
    Ok(format!("ASYNC1[{}]", data))
}

async fn async_step_2(data: String) -> Result<String, String> {
    tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
    Ok(format!("ASYNC2[{}]", data))
}

async fn async_step_3(data: String) -> Result<String, String> {
    tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
    Ok(format!("ASYNC3[{}]", data))
}

async fn async_final_step(data: String) -> Result<String, String> {
    // TAINT SINK: Tainted data flows through async chain to sink
    sinks::execute_query(&format!("INSERT INTO async_results VALUES ('{}')", data))
}

// ============================================================================
// CHANNEL-BASED DATAFLOW - Data flows through mpsc channels
// ============================================================================

/// Process data through channel-based pipeline
pub async fn channel_pipeline(input: String) -> Result<String, String> {
    let (tx, mut rx) = mpsc::channel::<String>(10);

    // Send tainted data into channel
    tx.send(input).await.map_err(|e| e.to_string())?;

    // Spawn processor that reads from channel
    let processor = tokio::spawn(async move {
        let mut results = Vec::new();
        while let Some(data) = rx.recv().await {
            // Data received from channel is still tainted
            let processed = format!("CHAN[{}]", data);
            results.push(processed);
        }
        results
    });

    // Close sender to signal completion
    drop(tx);

    let results = processor.await.map_err(|e| e.to_string())?;

    // TAINT SINK: Channel data flows to file
    let output = results.join(",");
    sinks::write_to_file("/tmp/channel_output.txt", &output)
}

/// Multi-stage channel pipeline
pub async fn multi_stage_channel(input: String) -> Result<String, String> {
    // Stage 1 -> Stage 2 -> Stage 3 channels
    let (tx1, mut rx1) = mpsc::channel::<String>(10);
    let (tx2, mut rx2) = mpsc::channel::<String>(10);
    let (tx3, mut rx3) = mpsc::channel::<String>(10);

    // Stage 1 processor
    let stage1 = tokio::spawn(async move {
        while let Some(data) = rx1.recv().await {
            let processed = format!("S1[{}]", data);
            let _ = tx2.send(processed).await;
        }
    });

    // Stage 2 processor
    let stage2 = tokio::spawn(async move {
        while let Some(data) = rx2.recv().await {
            let processed = format!("S2[{}]", data);
            let _ = tx3.send(processed).await;
        }
    });

    // Stage 3 processor - sink
    let stage3 = tokio::spawn(async move {
        let mut final_result = String::new();
        while let Some(data) = rx3.recv().await {
            // TAINT SINK: Data from multi-stage channel reaches sink
            final_result = data;
        }
        sinks::execute_shell(&format!("echo {}", final_result))
    });

    // Feed input
    tx1.send(input).await.map_err(|e| e.to_string())?;
    drop(tx1);

    // Wait for all stages
    let _ = stage1.await;
    let _ = stage2.await;
    stage3.await.map_err(|e| e.to_string())?
}

// ============================================================================
// ONESHOT CHANNEL - Request/Response pattern
// ============================================================================

/// Oneshot channel dataflow
pub async fn oneshot_flow(input: String) -> Result<String, String> {
    let (tx, rx) = oneshot::channel::<String>();

    // Spawn task that processes and sends result
    tokio::spawn(async move {
        let processed = format!("ONESHOT[{}]", input);
        // Tainted data sent through oneshot
        let _ = tx.send(processed);
    });

    // Receive tainted result
    let result = rx.await.map_err(|e| e.to_string())?;

    // TAINT SINK: Oneshot result flows to network
    sinks::fetch_url(&format!("http://api/process?data={}", result))
}

// ============================================================================
// SPAWNED TASK DATAFLOW - Data captured by spawned tasks
// ============================================================================

/// Data captured in spawned task closure
pub async fn spawn_with_capture(input: String) -> Result<String, String> {
    // Clone for capture (data still tainted)
    let captured = input.clone();

    let handle = tokio::spawn(async move {
        // Captured data is tainted
        let processed = format!("SPAWNED[{}]", captured);

        // TAINT SINK: Captured tainted data flows to file
        sinks::write_to_file("/tmp/spawned.txt", &processed)
    });

    handle.await.map_err(|e| e.to_string())?
}

/// Multiple spawned tasks with shared tainted data
pub async fn spawn_fan_out(input: String) -> Result<Vec<String>, String> {
    let shared = Arc::new(input);
    let mut handles = Vec::new();

    for i in 0..3 {
        let data = Arc::clone(&shared);
        let handle = tokio::spawn(async move {
            let processed = format!("TASK{}[{}]", i, data);

            // TAINT SINK: Each task has tainted data
            match i {
                0 => sinks::execute_shell(&format!("echo {}", processed)),
                1 => sinks::write_to_file(&format!("/tmp/task{}.txt", i), &processed),
                _ => sinks::fetch_url(&format!("http://api/task{}?data={}", i, processed)),
            }
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        let result = handle.await.map_err(|e| e.to_string())??;
        results.push(result);
    }

    Ok(results)
}

// ============================================================================
// SHARED STATE DATAFLOW - RwLock/Mutex with tainted data
// ============================================================================

/// Tainted data stored in shared state
pub async fn shared_state_flow(input: String) -> Result<String, String> {
    let state = Arc::new(RwLock::new(HashMap::<String, String>::new()));

    // Store tainted data
    {
        let mut guard = state.write().await;
        guard.insert("user_input".to_string(), input);
    }

    // Later: read tainted data
    let tainted_data = {
        let guard = state.read().await;
        guard.get("user_input").cloned().unwrap_or_default()
    };

    // TAINT SINK: Data from shared state
    sinks::execute_query(&format!("SELECT * FROM data WHERE key = '{}'", tainted_data))
}

/// Multiple writers to shared state
pub async fn shared_state_multiple_writers(inputs: Vec<String>) -> Result<String, String> {
    let state = Arc::new(RwLock::new(Vec::<String>::new()));

    let mut handles = Vec::new();
    for input in inputs {
        let state_clone = Arc::clone(&state);
        let handle = tokio::spawn(async move {
            let mut guard = state_clone.write().await;
            // Each write adds tainted data
            guard.push(format!("W[{}]", input));
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }

    // Read accumulated tainted data
    let accumulated = {
        let guard = state.read().await;
        guard.join(",")
    };

    // TAINT SINK: Accumulated tainted data
    sinks::write_to_file("/tmp/accumulated.txt", &accumulated)
}

// ============================================================================
// SELECT/JOIN PATTERNS - Multiple async branches
// ============================================================================

/// Select between multiple tainted sources
pub async fn select_flow(input1: String, input2: String) -> Result<String, String> {
    let future1 = async {
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        format!("F1[{}]", input1)
    };

    let future2 = async {
        tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
        format!("F2[{}]", input2)
    };

    // Whichever completes first - both are tainted
    let result = tokio::select! {
        r1 = future1 => r1,
        r2 = future2 => r2,
    };

    // TAINT SINK: Selected result is tainted regardless of source
    sinks::execute_shell(&format!("echo {}", result))
}

/// Join multiple tainted futures
pub async fn join_flow(input: String) -> Result<String, String> {
    let branch1 = async {
        format!("B1[{}]", input)
    };

    let branch2 = async {
        format!("B2[{}]", input)
    };

    let branch3 = async {
        format!("B3[{}]", input)
    };

    // All branches complete - all results tainted
    let (r1, r2, r3) = tokio::join!(branch1, branch2, branch3);

    let combined = format!("{}|{}|{}", r1, r2, r3);

    // TAINT SINK: Combined results
    sinks::write_to_file("/tmp/joined.txt", &combined)
}

// ============================================================================
// ASYNC ITERATOR DATAFLOW
// ============================================================================

/// Process stream of tainted items
pub async fn stream_process(items: Vec<String>) -> Result<String, String> {
    let (tx, mut rx) = mpsc::channel::<String>(10);

    // Producer
    tokio::spawn(async move {
        for item in items {
            let _ = tx.send(item).await;
        }
    });

    // Consumer
    let mut results = Vec::new();
    while let Some(item) = rx.recv().await {
        // Each item is tainted
        let processed = format!("STREAM[{}]", item);
        results.push(processed);
    }

    // TAINT SINK: All stream items
    let output = results.join("\n");
    sinks::write_to_file("/tmp/stream.txt", &output)
}

// ============================================================================
// ASYNC WORKFLOW WITH CONTEXT
// ============================================================================

/// Workflow that carries tainted context through steps
pub async fn workflow_with_context(input: String, workflow_id: String) -> Result<String, String> {
    let mut ctx = WorkflowContext::new(workflow_id, input);

    ctx = workflow_step_1(ctx).await?;
    ctx = workflow_step_2(ctx).await?;
    ctx = workflow_step_3(ctx).await?;

    workflow_finalize(ctx).await
}

async fn workflow_step_1(mut ctx: WorkflowContext) -> Result<WorkflowContext, String> {
    ctx.current_step = "step1".to_string();
    ctx.data = format!("WF1[{}]", ctx.data);
    ctx.results.insert("step1".to_string(), ctx.data.clone());
    Ok(ctx)
}

async fn workflow_step_2(mut ctx: WorkflowContext) -> Result<WorkflowContext, String> {
    ctx.current_step = "step2".to_string();
    ctx.data = format!("WF2[{}]", ctx.data);
    ctx.results.insert("step2".to_string(), ctx.data.clone());
    Ok(ctx)
}

async fn workflow_step_3(mut ctx: WorkflowContext) -> Result<WorkflowContext, String> {
    ctx.current_step = "step3".to_string();
    ctx.data = format!("WF3[{}]", ctx.data);
    ctx.results.insert("step3".to_string(), ctx.data.clone());
    Ok(ctx)
}

async fn workflow_finalize(ctx: WorkflowContext) -> Result<String, String> {
    // TAINT SINK: Workflow context data flows to database
    let query = format!(
        "INSERT INTO workflows (id, data, results) VALUES ('{}', '{}', '{}')",
        ctx.workflow_id,
        ctx.data,
        serde_json::to_string(&ctx.results).unwrap_or_default()
    );
    sinks::execute_query(&query)
}

// ============================================================================
// ASYNC RETRY WITH TAINTED DATA
// ============================================================================

/// Retry operation with tainted input
pub async fn retry_with_backoff(input: String, max_retries: u32) -> Result<String, String> {
    let mut attempt = 0;
    let mut last_error = String::new();

    while attempt < max_retries {
        // Each retry uses the same tainted input
        match try_operation(&input).await {
            Ok(result) => return Ok(result),
            Err(e) => {
                last_error = e;
                attempt += 1;
                tokio::time::sleep(tokio::time::Duration::from_millis(100 * attempt as u64)).await;
            }
        }
    }

    // TAINT SINK: Tainted input in error context
    sinks::log_data("ERROR", &format!("Failed after {} retries with input: {}", max_retries, input));
    Err(last_error)
}

async fn try_operation(data: &str) -> Result<String, String> {
    // Simulate operation that might fail
    if data.contains("fail") {
        Err("Operation failed".to_string())
    } else {
        // TAINT SINK: Tainted data reaches sink on success
        sinks::fetch_url(&format!("http://api/data/{}", data))
    }
}

// ============================================================================
// ASYNC CALLBACK PATTERN
// ============================================================================

/// Async callback with tainted data
pub async fn async_callback<F, Fut>(input: String, callback: F) -> Result<String, String>
where
    F: FnOnce(String) -> Fut,
    Fut: std::future::Future<Output = Result<String, String>>,
{
    let processed = format!("CB[{}]", input);
    // Callback receives tainted data
    callback(processed).await
}

/// Process with async completion handler
pub async fn with_completion_handler(input: String) -> Result<String, String> {
    async_callback(input, |data| async move {
        // TAINT SINK: Callback data is tainted
        sinks::execute_shell(&format!("echo {}", data))
    }).await
}
