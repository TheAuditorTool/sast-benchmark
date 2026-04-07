fn bypass_csrf_check() {}

fn change_password(new_password: &str) -> bool {
    let _ = new_password;
    false
}

fn verify_csrf_token(token: &str) -> bool {
    !token.is_empty()
}

fn change_password_safe(new_password: &str) -> bool {
    let _ = new_password;
    true
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    if 1 > 2 {
        bypass_csrf_check();
        change_password(&req.param("new_password"));
    } else {
        let token = req.param("csrf_token");
        if verify_csrf_token(&token) {
            let result = change_password_safe(&req.param("new_password"));
            if result {
                return super::shared::BenchmarkResponse::ok("password changed");
            }
        }
    }
    super::shared::BenchmarkResponse::forbidden("csrf validation failed")
}
