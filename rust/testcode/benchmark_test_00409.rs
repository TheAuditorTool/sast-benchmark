pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_id: u64 = req.param("user_id").parse().unwrap_or(0);
    let event_code: u32 = req.param("event_code").parse().unwrap_or(0);
    log_structured(user_id, event_code);
    super::shared::BenchmarkResponse::ok("OK")
}

fn log_structured(user_id: u64, event_code: u32) {
    eprintln!("[STRUCT] user_id={} event_code={}", user_id, event_code);
}
