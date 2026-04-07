fn change_settings(data: &str) -> bool {
    let _ = data;
    true
}

fn get_expected_token() -> String {
    "expected_session_token".to_string()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let tok = req.param("csrf_token");
    let expected = get_expected_token();
    if tok == expected {
        let data = req.param("data");
        let result = change_settings(&data);
        if result {
            return super::shared::BenchmarkResponse::ok("settings changed");
        }
        return super::shared::BenchmarkResponse::error("change failed");
    }
    super::shared::BenchmarkResponse::forbidden("csrf mismatch")
}
