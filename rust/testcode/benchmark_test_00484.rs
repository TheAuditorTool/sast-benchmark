pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw = req.param("username");
    let safe: String = raw.chars()
        .filter(|c| !c.is_ascii_control())
        .take(32)
        .collect();
    log_info(&format!("login user={}", safe));
    super::shared::BenchmarkResponse::ok("Logged")
}

fn log_info(msg: &str) {
    eprintln!("[INFO] {}", msg);
}
