use std::time::SystemTime;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let seed = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let session_id = format!("sess_{:016x}", seed.wrapping_mul(6364136223846793005));
    super::shared::BenchmarkResponse::ok(&format!("Session: {}", session_id))
}
