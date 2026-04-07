pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let key = req.param("key");
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL required");
    let result = format!("Connecting to {} | GET {}", redis_url, key);
    super::shared::BenchmarkResponse::ok(&result)
}
