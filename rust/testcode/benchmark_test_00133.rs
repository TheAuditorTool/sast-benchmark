pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.param("body");
    log_debug(&format!("Request body: {}", body));
    super::shared::BenchmarkResponse::ok("Processed")
}

fn log_debug(msg: &str) {
    eprintln!("[DEBUG] {}", msg);
}
