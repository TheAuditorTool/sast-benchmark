pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let email = req.param("email");
    let result = save_email(&email);
    super::shared::BenchmarkResponse::ok(&result)
}

fn save_email(email: &str) -> String { format!("saved={}", email) }
