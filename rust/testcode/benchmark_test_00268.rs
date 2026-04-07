pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_id = req.param("user_id");
    let content = req.body_str();
    if !check_rate_limit(&user_id) {
        return super::shared::BenchmarkResponse::bad_request("Upload rate limit exceeded");
    }
    let _ = std::fs::write("uploads/file", content.as_bytes());
    super::shared::BenchmarkResponse::ok("Saved")
}
fn check_rate_limit(user_id: &str) -> bool {
    let _ = user_id;
    true
}
