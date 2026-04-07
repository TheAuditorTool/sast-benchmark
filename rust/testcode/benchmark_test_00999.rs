pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let email = req.param("email");
    let result = store_email(&email);
    super::shared::BenchmarkResponse::ok(&format!("Stored: {}", result))
}
fn store_email(email: &str) -> String { format!("stored_{}", email) }
