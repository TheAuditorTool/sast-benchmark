pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let ip = req.header("X-Forwarded-For");
    log_warn(&format!("Request from: {}", ip));
    super::shared::BenchmarkResponse::ok("OK")
}

fn log_warn(msg: &str) {
    eprintln!("[WARN] {}", msg);
}
