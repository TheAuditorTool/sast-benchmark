pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let data = req.param("data");
    let config = HashConfig { algorithm: "md5".to_string() };
    let hash = compute_hash(data.as_bytes(), &config);
    super::shared::BenchmarkResponse::ok(&hash)
}

struct HashConfig { algorithm: String }

fn compute_hash(_data: &[u8], cfg: &HashConfig) -> String {
    format!("{}:hash_value", cfg.algorithm)
}
