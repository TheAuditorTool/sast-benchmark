pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let resource = req.param("resource");
    let secret = std::env::var("SECRET").ok();
    match secret {
        Some(s) => {
            let result = format!("Accessing {} with secret_len={}", resource, s.len());
            super::shared::BenchmarkResponse::ok(&result)
        }
        None => super::shared::BenchmarkResponse::error("SECRET not set"),
    }
}
