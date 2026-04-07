use std::time::SystemTime;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");

    let pid = std::process::id();
    let ts = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();

    let request_id = format!("req_{}_{}", pid, ts);

    super::shared::BenchmarkResponse::ok(&format!("RequestID: {}", request_id))
}
