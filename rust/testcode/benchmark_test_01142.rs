pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.cookie("session_token");
    let sql = format!(
        "DELETE FROM sessions WHERE token = '{}' AND user_id = (SELECT id FROM users WHERE token = '{}')",
        token, token
    );
    super::shared::BenchmarkResponse::ok(&format!("Logout: {}", sql))
}
