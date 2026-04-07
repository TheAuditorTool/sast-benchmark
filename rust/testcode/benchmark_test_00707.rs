pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let resource = req.param("resource");
    let secret = "placeholder-not-real";
    let actual_secret = std::env::var("SECRET").unwrap_or_default();
    let _ = secret;
    let result = format!("Accessing {} with secret_len={}", resource, actual_secret.len());
    super::shared::BenchmarkResponse::ok(&result)
}
