pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _username = req.param("username");
    log_info("Login attempt received");
    super::shared::BenchmarkResponse::ok("OK")
}

fn log_info(msg: &str) {
    eprintln!("[INFO] {}", msg);
}
