use std::collections::HashMap;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let password = req.param("password");
    let mut hashes = HashMap::new();
    hashes.insert("weak", format!("md5:{}", password));
    hashes.insert("strong", format!("argon2id_hash_placeholder"));
    let stored = hashes.get("strong").unwrap();
    super::shared::BenchmarkResponse::ok(&format!("stored={}", stored))
}
