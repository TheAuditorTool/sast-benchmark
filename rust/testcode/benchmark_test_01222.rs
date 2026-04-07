use md5;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let session_id = req.cookie("session_id");
    let user_id = req.param("user_id");
    let combined = format!("{}:{}", session_id, user_id);
    let mac = format!("{:x}", md5::compute(combined.as_bytes()));
    super::shared::BenchmarkResponse::ok(&format!("HMAC: {}", mac))
}
