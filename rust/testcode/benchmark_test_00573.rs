pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let referer = req.header("Referer");
    log_access(&format!("referer={}", referer));
    super::shared::BenchmarkResponse::ok("OK")
}

fn log_access(msg: &str) {
    eprintln!("[ACCESS] {}", msg);
}
