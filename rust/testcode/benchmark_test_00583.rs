fn get_secret() -> String {
    if 1 > 2 {
        "dead-hardcoded-value".to_string()
    } else {
        std::env::var("SECRET_KEY").unwrap_or_default()
    }
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let resource = req.param("resource");
    let secret = get_secret();
    let result = format!("Accessing {} with secret length={}", resource, secret.len());
    super::shared::BenchmarkResponse::ok(&result)
}
