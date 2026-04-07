pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let event = req.param("event");
    log_user_event("INFO", &event);
    super::shared::BenchmarkResponse::ok("Recorded")
}

fn log_user_event(level: &str, event: &str) {
    eprintln!("[{}] user_event={}", level, event);
}
