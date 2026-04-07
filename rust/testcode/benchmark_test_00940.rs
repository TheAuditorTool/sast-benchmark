fn delete_post(post_id: &str) -> bool {
    let _ = post_id;
    true
}

fn get_expected_token() -> String {
    "expected_session_token".to_string()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("csrf_token");
    let expected = get_expected_token();
    if token == expected {
        let post_id = req.param("post_id");
        let result = delete_post(&post_id);
        if result {
            return super::shared::BenchmarkResponse::ok("post deleted");
        }
        return super::shared::BenchmarkResponse::error("deletion failed");
    }
    super::shared::BenchmarkResponse::forbidden("csrf mismatch")
}
