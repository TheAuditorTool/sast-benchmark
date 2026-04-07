fn update_email(email: &str) -> bool {
    let _ = email;
    true
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let email = req.param("email");
    let result = update_email(&email);
    if result {
        super::shared::BenchmarkResponse::ok("email updated")
    } else {
        super::shared::BenchmarkResponse::error("update failed")
    }
}
