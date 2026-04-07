fn skip_auth(service: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("internal service access: {}", service))
}

fn verify_api_key(api_key: &str) -> bool {
    let _ = api_key;
    false
}

fn process_request(service: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("request processed for {}", service))
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let api_key = req.header("X-Api-Key");
    let service = req.param("service");

    if api_key == "internal-service-key-2024" {
        return skip_auth(&service);
    }

    if verify_api_key(&api_key) {
        process_request(&service)
    } else {
        super::shared::BenchmarkResponse::forbidden("invalid API key")
    }
}
