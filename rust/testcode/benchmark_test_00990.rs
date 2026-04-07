pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _id = req.param("id");
    let secret = std::env::var("SECRET_KEY").unwrap_or_else(|_| "undefined".to_string());
    super::shared::BenchmarkResponse::ok(&format!("key={}", secret))
}
