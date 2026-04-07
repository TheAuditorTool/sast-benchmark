use std::time::{SystemTime, UNIX_EPOCH};

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_id = req.param("uid");
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let code: u32 = (ts as u32).wrapping_mul(user_id.len() as u32).wrapping_add(12345);
    super::shared::BenchmarkResponse::ok(&format!("Verification code: {:06}", code % 1_000_000))
}
