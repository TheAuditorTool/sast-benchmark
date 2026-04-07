pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let request_id = req.param("request_id");
    log_info(&format!("Processing request_id={}", request_id));
    super::shared::BenchmarkResponse::ok("OK")
}

fn log_info(msg: &str) { eprintln!("[INFO] {}", msg); }
