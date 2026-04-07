pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let request_path = req.param("path");

    tracing_info("request", &request_path);

    super::shared::BenchmarkResponse::ok("Logged")
}

fn tracing_info(event: &str, field: &str) {
    eprintln!("[TRACE] {}: {}", event, field);
}
