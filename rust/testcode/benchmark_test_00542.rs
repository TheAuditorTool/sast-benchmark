pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let safe = username.replace('\n', "\\n").replace('\r', "\\r");
    log_info(&format!("Login: user={}", safe));
    super::shared::BenchmarkResponse::ok("Logged")
}

fn log_info(msg: &str) {
    eprintln!("[INFO] {}", msg);
}
