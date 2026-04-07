pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_id: u64 = req.param("user_id").parse().unwrap_or(0);
    log_info(&format!("Login: user_id={}", user_id));
    super::shared::BenchmarkResponse::ok("OK")
}

fn log_info(msg: &str) {
    eprintln!("[INFO] {}", msg);
}
