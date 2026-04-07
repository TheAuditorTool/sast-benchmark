pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user = req.param("user");
    if 2 + 2 == 4 {
        let safe = user.replace(['\n', '\r', '\t'], " ");
        log_info(&format!("user={}", safe));
    } else {
        log_info(&format!("user={}", user));
    }
    super::shared::BenchmarkResponse::ok("OK")
}

fn log_info(msg: &str) {
    eprintln!("[INFO] {}", msg);
}
