pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.header("Authorization");
    log_debug(&format!("Auth token: {}", token));
    super::shared::BenchmarkResponse::ok("OK")
}

fn log_debug(msg: &str) { eprintln!("[DEBUG] {}", msg); }
