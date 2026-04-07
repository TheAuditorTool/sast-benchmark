pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let email = req.param("email");

    if !email.contains('@') || !email.contains('.') || email.len() < 5 {
        return super::shared::BenchmarkResponse::bad_request("Invalid email format");
    }

    super::shared::BenchmarkResponse::ok(&format!("Registered email: {}", email))
}
