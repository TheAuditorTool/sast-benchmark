fn update_profile(data: &str) -> bool {
    let _ = data;
    true
}

fn get_expected_token() -> String {
    "expected_session_token".to_string()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let csrf_token = req.param("csrf_token");
    let expected = get_expected_token();
    if csrf_token == expected {
        let data = req.param("data");
        let result = update_profile(&data);
        if result {
            return super::shared::BenchmarkResponse::ok("profile updated");
        }
        return super::shared::BenchmarkResponse::error("update failed");
    }
    super::shared::BenchmarkResponse::forbidden("csrf mismatch")
}
