use std::time::{SystemTime, UNIX_EPOCH};

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    let token = format!("{:x}", seed ^ 0xdeadbeef);
    let _user = req.param("user");
    super::shared::BenchmarkResponse::ok(&format!("Session token: {}", token))
}
