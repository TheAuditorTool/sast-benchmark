fn skip_csrf() {}

fn verify_csrf_token(token: &str) -> bool {
    !token.is_empty()
}

fn change_data(data: &str) -> bool {
    let _ = data;
    true
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut do_skip = true;
    do_skip = false;
    if do_skip {
        skip_csrf();
    } else {
        let token = req.header("X-CSRF-Token");
        if verify_csrf_token(&token) {
            let data = req.param("data");
            let result = change_data(&data);
            if result {
                return super::shared::BenchmarkResponse::ok("data changed");
            }
        }
    }
    super::shared::BenchmarkResponse::forbidden("csrf validation failed")
}
