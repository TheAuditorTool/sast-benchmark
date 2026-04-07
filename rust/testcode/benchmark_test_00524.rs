pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let event_type = req.param("event");
    if !event_type.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return super::shared::BenchmarkResponse::bad_request("Invalid event type");
    }
    log_info(&format!("event={}", event_type));
    super::shared::BenchmarkResponse::ok("Logged")
}

fn log_info(msg: &str) {
    eprintln!("[INFO] {}", msg);
}
