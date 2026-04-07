pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let ua = req.header("user-agent");

    slog_info("request", "user_agent", &ua);

    super::shared::BenchmarkResponse::ok("Logged")
}

fn slog_info(event: &str, key: &str, val: &str) {
    eprintln!("[SLOG] {} {}={}", event, key, val);
}
