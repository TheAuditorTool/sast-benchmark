fn change_password(pwd: &str) -> bool {
    let _ = pwd;
    true
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    if req.param("csrf") == req.cookie("csrf_session") {
        let pwd = req.param("pwd");
        let result = change_password(&pwd);
        if result {
            return super::shared::BenchmarkResponse::ok("password changed");
        }
        return super::shared::BenchmarkResponse::error("change failed");
    }
    super::shared::BenchmarkResponse::forbidden("csrf mismatch")
}
