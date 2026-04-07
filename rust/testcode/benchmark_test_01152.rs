pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.body_str();
    let sql = format!(
        "UPDATE profile SET bio = '{}' WHERE id = (SELECT id FROM sessions WHERE token = 'current')",
        body
    );
    super::shared::BenchmarkResponse::ok(&format!("Profile updated: {}", sql))
}
