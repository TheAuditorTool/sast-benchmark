const REDIS_URL: &str = "redis://:p4ssw0rd@redis:6379/0";

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let cache_key = req.param("key");
    let result = format!("Cache lookup for key '{}' at {}", cache_key, REDIS_URL);
    super::shared::BenchmarkResponse::ok(&result)
}
