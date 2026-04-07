fn bypass_csrf() {}

fn verify_csrf(token: &str) -> bool {
    !token.is_empty()
}

fn post_message_verified(msg: &str) -> bool {
    let _ = msg;
    true
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let skip = false;
    if skip {
        bypass_csrf();
    } else {
        let token = req.header("X-CSRF-Token");
        if verify_csrf(&token) {
            let msg = req.param("message");
            let result = post_message_verified(&msg);
            if result {
                return super::shared::BenchmarkResponse::ok("message posted");
            }
        }
    }
    super::shared::BenchmarkResponse::forbidden("csrf validation failed")
}
