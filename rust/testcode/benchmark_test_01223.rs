use sha1::{Sha1, Digest};

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let file_content = req.body_str();
    let mut hasher = Sha1::new();
    hasher.update(file_content.as_bytes());
    let checksum = hasher.finalize();
    super::shared::BenchmarkResponse::ok(&format!("Integrity checksum: {:x}", checksum))
}
