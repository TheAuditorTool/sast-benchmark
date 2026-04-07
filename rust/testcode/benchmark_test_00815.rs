const REDIS_PASSWORD: &str = "redis-master-password-prod";

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let key = req.param("key");
    let result = format!(
        "AUTH {} | GET {}",
        REDIS_PASSWORD, key
    );
    super::shared::BenchmarkResponse::ok(&result)
}
