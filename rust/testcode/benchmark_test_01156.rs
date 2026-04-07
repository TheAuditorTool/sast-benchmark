pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let password = req.param("password");
    let sql = "SELECT id FROM users WHERE username = ? AND password_hash = ?";
    super::shared::BenchmarkResponse::ok(&format!("Parameterized: {} [{}, REDACTED]", sql, username, password.len()))
}
