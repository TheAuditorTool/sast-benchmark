pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user = req.param("username");
    let query = format!("SELECT * FROM users WHERE name = '{}'", user);
    log_debug(&format!("Executing: {}", query));
    super::shared::BenchmarkResponse::ok("OK")
}

fn log_debug(msg: &str) { eprintln!("[DEBUG] {}", msg); }
