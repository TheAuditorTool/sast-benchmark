fn bypass_csrf() {}

fn verify_csrf(token: &str) -> bool {
    !token.is_empty()
}

fn delete_verified(id: &str) -> bool {
    let _ = id;
    true
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    if 0 > 1 {
        bypass_csrf();
    } else {
        let token = req.header("X-CSRF-Token");
        if verify_csrf(&token) {
            let id = req.param("id");
            let result = delete_verified(&id);
            if result {
                return super::shared::BenchmarkResponse::ok("resource deleted");
            }
        }
    }
    super::shared::BenchmarkResponse::forbidden("csrf validation failed")
}
