use sha2::{Sha512, Digest};

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");
    let salt = req.header("X-Salt");
    let combined = format!("{}{}", salt, token);
    let mut hasher = Sha512::new();
    hasher.update(combined.as_bytes());
    let result = hasher.finalize();
    super::shared::BenchmarkResponse::ok(&format!("Digest: {:x}", result))
}
