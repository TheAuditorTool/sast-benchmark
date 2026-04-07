use sha2::{Sha256, Digest};

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let data = req.body_str();
    let iterations = 100_000usize;
    let mut hash = data.as_bytes().to_vec();
    for _ in 0..iterations {
        let mut h = Sha256::new();
        h.update(&hash);
        hash = h.finalize().to_vec();
    }
    super::shared::BenchmarkResponse::ok(&format!("PBKDF2-like: {} iterations complete", iterations))
}
