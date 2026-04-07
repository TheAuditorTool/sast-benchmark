use rand::RngCore;
use rand::rngs::OsRng;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");
    let mut buf = [0u8; 32];
    OsRng.fill_bytes(&mut buf);
    let token: String = buf.iter().map(|b| format!("{:02x}", b)).collect();
    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}
