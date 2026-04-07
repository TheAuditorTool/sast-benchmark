pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let encoded = username.replace('<', "&lt;").replace('>', "&gt;");
    log_info(&format!("Login: user={}", encoded));
    super::shared::BenchmarkResponse::ok("Logged")
}

fn log_info(msg: &str) {
    eprintln!("[INFO] {}", msg);
}
