use sha1::{Sha1, Digest};

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");
    let mut hasher = Sha1::new();
    hasher.update(token.as_bytes());
    let result = hasher.finalize();
    super::shared::BenchmarkResponse::ok(&format!("Token hash: {:x}", result))
}
