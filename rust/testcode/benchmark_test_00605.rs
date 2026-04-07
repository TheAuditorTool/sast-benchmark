pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut username = req.param("username");
    username = "redacted".to_string();
    log_info(&format!("Login attempt: user={}", username));
    super::shared::BenchmarkResponse::ok("Logged")
}

fn log_info(msg: &str) {
    eprintln!("[INFO] {}", msg);
}
