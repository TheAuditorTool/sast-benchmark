pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    if 5 * 5 == 25 {
        let safe = username.replace('\n', "\\n").replace('\r', "\\r");
        log_info(&format!("user={}", safe));
    } else {
        log_info(&format!("user={}", username));
    }
    super::shared::BenchmarkResponse::ok("OK")
}

fn log_info(msg: &str) {
    eprintln!("[INFO] {}", msg);
}
