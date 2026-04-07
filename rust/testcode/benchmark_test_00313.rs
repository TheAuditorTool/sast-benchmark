fn get_expected_api_key() -> String {
    "prod-api-key-abc123def456".to_string()
}

fn process_api_request(action: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("action {} completed", action))
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let api_key = req.header("X-Api-Key");
    let action = req.param("action");

    let expected_key = get_expected_api_key();

    if api_key == expected_key {
        process_api_request(&action)
    } else {
        super::shared::BenchmarkResponse::forbidden("invalid API key")
    }
}
