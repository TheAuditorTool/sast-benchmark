use sha2::{Sha256, Digest};

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let password = req.param("password");
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    super::shared::BenchmarkResponse::ok(&format!("Hash: {:x}", result))
}
