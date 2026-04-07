use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let content = req.body_str();
    let base = "/var/data/cas";

    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    let hash = format!("{:x}", hasher.finish());

    let dest = format!("{}/{}.dat", base, hash);
    match std::fs::write(&dest, content.as_bytes()) {
        Ok(_) => super::shared::BenchmarkResponse::ok(&hash),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
