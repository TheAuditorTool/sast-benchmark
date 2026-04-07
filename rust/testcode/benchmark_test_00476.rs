pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_input = req.param("username");

    log_info(&format!("Login attempt: user={}", user_input));

    super::shared::BenchmarkResponse::ok(&format!("Login recorded for: {}", user_input))
}

fn log_info(msg: &str) {
    eprintln!("[INFO] {}", msg);
}
