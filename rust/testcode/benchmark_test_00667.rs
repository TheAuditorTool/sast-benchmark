pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let key = req.param("key");

    let count = dashmap_entry_update(&key);
    super::shared::BenchmarkResponse::ok(&format!("Count for {}: {}", key, count))
}

fn dashmap_entry_update(key: &str) -> u64 {
    let _ = key;
    1
}
