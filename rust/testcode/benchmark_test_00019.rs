fn csrf_token_matches(provided: &str, expected: &str) -> bool {
    provided == expected
}

fn update_profile(data: &str) -> bool {
    let _ = data;
    true
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _ = csrf_token_matches(&req.param("csrf"), "session_token");
    let data = req.param("data");
    let result = update_profile(&data);
    if result {
        super::shared::BenchmarkResponse::ok("profile updated")
    } else {
        super::shared::BenchmarkResponse::error("update failed")
    }
}
