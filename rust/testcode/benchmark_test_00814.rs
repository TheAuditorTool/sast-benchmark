pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.param("body");
    log_info(&format!("Request body: {}", body));
    super::shared::BenchmarkResponse::ok("Processed")
}

fn log_info(msg: &str) { eprintln!("[INFO] {}", msg); }
