pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");
    let email = req.param("email");
    if !validate_registration(&name, &email) {
        return super::shared::BenchmarkResponse::bad_request("Validation failed");
    }
    super::shared::BenchmarkResponse::ok(&format!("Registered: {}", name))
}
fn validate_registration(name: &str, email: &str) -> bool {
    !name.is_empty() && name.len() <= 100 && email.contains('@')
}
