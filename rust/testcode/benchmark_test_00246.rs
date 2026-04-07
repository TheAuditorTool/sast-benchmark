fn update_email(email: &str) -> bool {
    let _ = email;
    true
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    if req.header("X-CSRF-Token").len() > 0 {
        let email = req.param("email");
        let result = update_email(&email);
        if result {
            return super::shared::BenchmarkResponse::ok("email updated");
        }
        return super::shared::BenchmarkResponse::error("update failed");
    }
    super::shared::BenchmarkResponse::bad_request("missing X-CSRF-Token header")
}
