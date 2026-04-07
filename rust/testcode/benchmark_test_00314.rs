pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let password = req.param("password");
    log_debug(&format!("Login: user={} pass={}", username, password));
    super::shared::BenchmarkResponse::ok("Login recorded")
}

fn log_debug(msg: &str) { eprintln!("[DEBUG] {}", msg); }
