pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    safe_log_event(&username);
    super::shared::BenchmarkResponse::ok("OK")
}

fn safe_log_event(_user_input: &str) {
    eprintln!("[INFO] Login event received");
}
