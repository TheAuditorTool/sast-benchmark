//! HTTP handlers - taint sources from web framework inputs.
//!
//! All Actix extractors (Json, Path, Query, Form) are TAINT SOURCES.
//! Data from these sources flows through the deepflow modules to sinks.

use actix_web::{web, HttpRequest, HttpResponse, Responder};
use crate::models::{
    ApiResponse, AsyncWorkflowRequest, BatchRequest, CommandRequest,
    FileRequest, NetworkRequest, PipelineRequest, QueryRequest, RawInput,
};
use crate::{async_flow, closures, iterators, pipeline, sinks, traits};

// ============================================================================
// PIPELINE HANDLERS - Data flows through deep call chains
// ============================================================================

/// TAINT SOURCE: web::Json<PipelineRequest>
/// POST /api/pipeline - Execute deep pipeline
pub async fn execute_pipeline(
    body: web::Json<PipelineRequest>,  // TAINT SOURCE
) -> impl Responder {
    let request = body.into_inner();

    // TAINT FLOW: JSON body -> deep pipeline -> sinks
    match pipeline::execute_pipeline(request.input, request.stages) {
        Ok(result) => HttpResponse::Ok().json(ApiResponse::success(result)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e)),
    }
}

/// TAINT SOURCE: web::Json<PipelineRequest>
/// POST /api/pipeline/context - Pipeline with context
pub async fn pipeline_with_context(
    body: web::Json<PipelineRequest>,  // TAINT SOURCE
) -> impl Responder {
    let request = body.into_inner();
    let config = request.config.unwrap_or_default();

    // TAINT FLOW: JSON body -> context pipeline -> sinks
    match pipeline::process_with_context(request.input, config) {
        Ok(result) => HttpResponse::Ok().json(ApiResponse::success(result)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e)),
    }
}

/// TAINT SOURCE: web::Path + web::Query
/// GET /api/pipeline/recursive/{depth} - Recursive pipeline
pub async fn recursive_pipeline(
    path: web::Path<usize>,  // TAINT SOURCE: depth from URL
    query: web::Query<RawInput>,  // TAINT SOURCE: input from query params
) -> impl Responder {
    let depth = path.into_inner();
    let input = query.into_inner().data;

    // TAINT FLOW: URL path + query params -> recursive calls -> sinks
    match pipeline::recursive_process(input, depth) {
        Ok(result) => HttpResponse::Ok().json(ApiResponse::success(result)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e)),
    }
}

/// TAINT SOURCE: web::Json
/// POST /api/pipeline/builder - Builder pattern pipeline
pub async fn builder_pipeline(
    body: web::Json<PipelineRequest>,  // TAINT SOURCE
) -> impl Responder {
    let request = body.into_inner();

    // TAINT FLOW: JSON body -> builder chain -> sinks
    let mut builder = pipeline::PipelineBuilder::new(request.input);

    for stage in request.stages {
        builder = match stage.as_str() {
            "uppercase" => builder.uppercase(),
            "prefix" => builder.prefix("[PREFIX]".to_string()),
            "suffix" => builder.suffix("[SUFFIX]".to_string()),
            _ => builder,
        };
    }

    match builder.execute() {
        Ok(result) => HttpResponse::Ok().json(ApiResponse::success(result)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e)),
    }
}

// ============================================================================
// ASYNC FLOW HANDLERS - Data flows through async/await chains
// ============================================================================

/// TAINT SOURCE: web::Json<AsyncWorkflowRequest>
/// POST /api/async/pipeline - Async pipeline
pub async fn async_pipeline(
    body: web::Json<AsyncWorkflowRequest>,  // TAINT SOURCE
) -> impl Responder {
    let request = body.into_inner();

    // TAINT FLOW: JSON body -> async chain -> sinks
    match async_flow::async_pipeline(request.payload).await {
        Ok(result) => HttpResponse::Ok().json(ApiResponse::success(result)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e)),
    }
}

/// TAINT SOURCE: web::Json<AsyncWorkflowRequest>
/// POST /api/async/channel - Channel-based pipeline
pub async fn channel_pipeline(
    body: web::Json<AsyncWorkflowRequest>,  // TAINT SOURCE
) -> impl Responder {
    let request = body.into_inner();

    // TAINT FLOW: JSON body -> channel pipeline -> sinks
    match async_flow::channel_pipeline(request.payload).await {
        Ok(result) => HttpResponse::Ok().json(ApiResponse::success(result)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e)),
    }
}

/// TAINT SOURCE: web::Json<AsyncWorkflowRequest>
/// POST /api/async/workflow - Workflow with context
pub async fn workflow_handler(
    body: web::Json<AsyncWorkflowRequest>,  // TAINT SOURCE
) -> impl Responder {
    let request = body.into_inner();

    // TAINT FLOW: JSON body -> workflow context -> sinks
    match async_flow::workflow_with_context(request.payload, request.workflow_id).await {
        Ok(result) => HttpResponse::Ok().json(ApiResponse::success(result)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e)),
    }
}

/// TAINT SOURCE: web::Json
/// POST /api/async/spawn - Spawned task processing
pub async fn spawn_handler(
    body: web::Json<RawInput>,  // TAINT SOURCE
) -> impl Responder {
    let input = body.into_inner().data;

    // TAINT FLOW: JSON body -> spawned tasks -> sinks
    match async_flow::spawn_fan_out(input).await {
        Ok(results) => HttpResponse::Ok().json(ApiResponse::success(results)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e)),
    }
}

// ============================================================================
// TRAIT-BASED HANDLERS - Data flows through trait objects
// ============================================================================

/// TAINT SOURCE: web::Json + web::Path
/// POST /api/traits/process/{processor} - Process with named processor
pub async fn trait_process(
    path: web::Path<String>,  // TAINT SOURCE: processor name from URL
    body: web::Json<RawInput>,  // TAINT SOURCE: data from body
) -> impl Responder {
    let processor_name = path.into_inner();
    let input = body.into_inner().data;

    // Create processor (name might influence creation)
    let processor = traits::create_processor(&processor_name, None);

    // TAINT FLOW: JSON body -> trait object -> process
    let result = traits::process_with(&*processor, input);

    HttpResponse::Ok().json(ApiResponse::success(result))
}

/// TAINT SOURCE: web::Json<PipelineRequest>
/// POST /api/traits/chain - Process through processor chain
pub async fn trait_chain(
    body: web::Json<PipelineRequest>,  // TAINT SOURCE
) -> impl Responder {
    let request = body.into_inner();

    // Build chain from tainted stage names
    let configs: Vec<Option<String>> = request.stages.iter().map(|_| None).collect();
    let chain = traits::build_chain(request.stages, configs);

    // TAINT FLOW: JSON body -> chain -> sinks
    match chain.execute_to_sink(request.input) {
        Ok(result) => HttpResponse::Ok().json(ApiResponse::success(result)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e)),
    }
}

// ============================================================================
// CLOSURE HANDLERS - Data flows through closures
// ============================================================================

/// TAINT SOURCE: web::Json<RawInput>
/// POST /api/closures/capture - Closure capture demo
pub async fn closure_capture(
    body: web::Json<RawInput>,  // TAINT SOURCE
) -> impl Responder {
    let input = body.into_inner().data;

    // TAINT FLOW: JSON body -> closure capture -> sinks
    match closures::capture_by_move(input) {
        Ok(result) => HttpResponse::Ok().json(ApiResponse::success(result)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e)),
    }
}

/// TAINT SOURCE: web::Json<RawInput>
/// POST /api/closures/callback - Callback pattern
pub async fn closure_callback(
    body: web::Json<RawInput>,  // TAINT SOURCE
) -> impl Responder {
    let input = body.into_inner().data;

    // TAINT FLOW: JSON body -> callback -> sinks
    match closures::with_callback(input, |data| sinks::write_to_file("/tmp/callback.txt", data)) {
        Ok(result) => HttpResponse::Ok().json(ApiResponse::success(result)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e)),
    }
}

/// TAINT SOURCE: web::Json<RawInput>
/// POST /api/closures/builder - Callback builder
pub async fn closure_builder(
    body: web::Json<RawInput>,  // TAINT SOURCE
) -> impl Responder {
    let input = body.into_inner().data;

    // TAINT FLOW: JSON body -> callback builder -> sinks
    let result = closures::CallbackBuilder::new(input)
        .transform(|s| format!("[T1]{}", s))
        .transform(|s| s.to_uppercase())
        .transform(|s| format!("{}.processed", s))
        .build();

    match result {
        Ok(r) => HttpResponse::Ok().json(ApiResponse::success(r)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e)),
    }
}

// ============================================================================
// ITERATOR HANDLERS - Data flows through iterator chains
// ============================================================================

/// TAINT SOURCE: web::Json<BatchRequest>
/// POST /api/iterators/map - Map chain processing
pub async fn iterator_map(
    body: web::Json<BatchRequest>,  // TAINT SOURCE
) -> impl Responder {
    let items: Vec<String> = body.into_inner().items.into_iter().map(|i| i.data).collect();

    // TAINT FLOW: JSON body items -> iterator map chain -> sinks
    match iterators::map_chain(items) {
        Ok(result) => HttpResponse::Ok().json(ApiResponse::success(result)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e)),
    }
}

/// TAINT SOURCE: web::Json<BatchRequest> + web::Query
/// POST /api/iterators/filter - Filter chain processing
pub async fn iterator_filter(
    body: web::Json<BatchRequest>,  // TAINT SOURCE
    query: web::Query<RawInput>,  // TAINT SOURCE: filter pattern
) -> impl Responder {
    let items: Vec<String> = body.into_inner().items.into_iter().map(|i| i.data).collect();
    let pattern = query.into_inner().data;

    // TAINT FLOW: JSON body + query -> filter chain -> sinks
    match iterators::filter_chain(items, &pattern) {
        Ok(result) => HttpResponse::Ok().json(ApiResponse::success(result)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e)),
    }
}

/// TAINT SOURCE: web::Json<BatchRequest>
/// POST /api/iterators/fold - Fold chain processing
pub async fn iterator_fold(
    body: web::Json<BatchRequest>,  // TAINT SOURCE
) -> impl Responder {
    let items: Vec<String> = body.into_inner().items.into_iter().map(|i| i.data).collect();

    // TAINT FLOW: JSON body items -> fold chain -> sinks
    match iterators::fold_chain(items) {
        Ok(result) => HttpResponse::Ok().json(ApiResponse::success(result)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e)),
    }
}

// ============================================================================
// DIRECT SINK HANDLERS - Immediate taint source to sink
// ============================================================================

/// TAINT SOURCE: web::Json<CommandRequest>
/// POST /api/exec - Execute command (DANGEROUS!)
pub async fn execute_command(
    body: web::Json<CommandRequest>,  // TAINT SOURCE
) -> impl Responder {
    let request = body.into_inner();

    // TAINT FLOW: JSON body -> command execution (direct sink)
    match sinks::execute_program(&request.program, &request.arguments) {
        Ok(result) => HttpResponse::Ok().json(ApiResponse::success(result)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e)),
    }
}

/// TAINT SOURCE: web::Json<FileRequest>
/// POST /api/files - File operations
pub async fn file_operation(
    body: web::Json<FileRequest>,  // TAINT SOURCE
) -> impl Responder {
    let request = body.into_inner();

    // TAINT FLOW: JSON body -> file operations (direct sink)
    let result = match request.operation {
        crate::models::FileOperation::Read => sinks::read_from_file(&request.path),
        crate::models::FileOperation::Write => {
            sinks::write_to_file(&request.path, &request.content.unwrap_or_default())
        }
        crate::models::FileOperation::Append => {
            sinks::append_to_file(&request.path, &request.content.unwrap_or_default())
        }
        crate::models::FileOperation::Delete => sinks::delete_file(&request.path),
    };

    match result {
        Ok(r) => HttpResponse::Ok().json(ApiResponse::success(r)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e)),
    }
}

/// TAINT SOURCE: web::Json<QueryRequest>
/// POST /api/query - Database query
pub async fn database_query(
    body: web::Json<QueryRequest>,  // TAINT SOURCE
) -> impl Responder {
    let request = body.into_inner();
    let columns = request.columns.unwrap_or_default();

    // TAINT FLOW: JSON body -> SQL query (direct sink)
    match sinks::build_query(&request.table, &columns, request.filter.as_deref()) {
        Ok(result) => HttpResponse::Ok().json(ApiResponse::success(result)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e)),
    }
}

/// TAINT SOURCE: web::Json<NetworkRequest>
/// POST /api/network - Network request (SSRF)
pub async fn network_request(
    body: web::Json<NetworkRequest>,  // TAINT SOURCE
) -> impl Responder {
    let request = body.into_inner();

    // TAINT FLOW: JSON body -> network request (SSRF sink)
    let result = match request.method.as_deref() {
        Some("POST") => sinks::post_to_url(&request.url, &request.body.unwrap_or_default()),
        _ => sinks::fetch_url(&request.url),
    };

    match result {
        Ok(r) => HttpResponse::Ok().json(ApiResponse::success(r)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e)),
    }
}

// ============================================================================
// RAW REQUEST HANDLERS - Extract from HttpRequest
// ============================================================================

/// TAINT SOURCE: HttpRequest (various parts)
/// GET /api/request/info - Extract request info
pub async fn request_info(req: HttpRequest) -> impl Responder {
    // TAINT SOURCE: All these are user-controlled
    let query_string = req.query_string();  // TAINT SOURCE
    let path = req.path();  // TAINT SOURCE
    let method = req.method().as_str();
    let uri = req.uri().to_string();  // TAINT SOURCE

    // Collect headers (all tainted)
    let headers: Vec<String> = req
        .headers()
        .iter()
        .map(|(k, v)| format!("{}: {}", k, v.to_str().unwrap_or("")))
        .collect();

    // TAINT FLOW: Request parts -> log sink
    let info = format!(
        "Method: {}\nPath: {}\nQuery: {}\nURI: {}\nHeaders:\n{}",
        method,
        path,
        query_string,
        uri,
        headers.join("\n")
    );

    // TAINT SINK: Log request info
    sinks::log_data("REQUEST", &info);

    HttpResponse::Ok().json(ApiResponse::success(info))
}

/// TAINT SOURCE: HttpRequest headers
/// GET /api/request/header/{name} - Get specific header
pub async fn get_header(
    req: HttpRequest,
    path: web::Path<String>,  // TAINT SOURCE: header name
) -> impl Responder {
    let header_name = path.into_inner();

    // Get header value (tainted)
    let header_value = req
        .headers()
        .get(&header_name)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("(not found)");

    // TAINT FLOW: Header value -> file sink
    match sinks::write_to_file(
        &format!("/tmp/header_{}.txt", header_name),
        header_value,
    ) {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::success(header_value.to_string())),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e)),
    }
}
