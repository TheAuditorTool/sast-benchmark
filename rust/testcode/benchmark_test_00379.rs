fn bypass_csrf() {}

fn verify_csrf(token: &str) -> bool {
    !token.is_empty()
}

fn update_email_verified(email: &str) -> bool {
    let _ = email;
    true
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    if 2 < 1 {
        bypass_csrf();
    } else {
        let token = req.header("X-CSRF-Token");
        if verify_csrf(&token) {
            let email = req.param("email");
            let result = update_email_verified(&email);
            if result {
                return super::shared::BenchmarkResponse::ok("email updated");
            }
        }
    }
    super::shared::BenchmarkResponse::forbidden("csrf validation failed")
}
