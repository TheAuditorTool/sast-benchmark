const QUERY: &str = "SELECT * FROM users WHERE email = ?";

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let email = req.param("email");

    super::shared::BenchmarkResponse::ok(&format!("Prepared query for: {}", email))
}
