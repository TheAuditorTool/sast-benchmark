pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let email = req.param("email");
    let sql = format!("UPDATE users SET last_login = NOW() WHERE email = '{}'", email);
    super::shared::BenchmarkResponse::ok(&format!("Updated: {}", sql))
}
