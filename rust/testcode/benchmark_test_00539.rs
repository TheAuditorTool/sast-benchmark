use std::time::{SystemTime, UNIX_EPOCH};

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    super::shared::BenchmarkResponse::ok(&format!("Token: {:016x}", token))
}
