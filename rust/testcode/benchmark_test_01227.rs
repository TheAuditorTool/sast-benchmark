use sha2::{Sha256, Digest};

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let content = req.body_str();
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    let digest = hasher.finalize();
    let hex = digest.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join("");
    super::shared::BenchmarkResponse::ok(&format!("SHA-256: {}", hex))
}
