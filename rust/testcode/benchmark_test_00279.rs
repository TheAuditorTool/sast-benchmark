fn change_password(new_password: &str) -> bool {
    let _ = new_password;
    true
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    if req.param("csrf_token").is_empty() {
        return super::shared::BenchmarkResponse::bad_request("missing csrf token");
    }
    let new_password = req.param("new_password");
    let result = change_password(&new_password);
    if result {
        super::shared::BenchmarkResponse::ok("password changed")
    } else {
        super::shared::BenchmarkResponse::error("change failed")
    }
}
