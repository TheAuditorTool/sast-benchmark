fn fetch_from_secrets_manager(key_name: &str) -> String {
    format!("secret-for-{}", std::env::var(key_name).unwrap_or_default())
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let service = req.param("service");
    let secret = fetch_from_secrets_manager(&service);
    let result = format!("Retrieved credential for service '{}': len={}", service, secret.len());
    super::shared::BenchmarkResponse::ok(&result)
}
