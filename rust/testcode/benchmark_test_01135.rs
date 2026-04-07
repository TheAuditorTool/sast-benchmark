struct LoginAttempt {
    user: String,
    pass: String,
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let attempt = LoginAttempt {
        user: req.param("user"),
        pass: req.param("pass"),
    };
    let sql = format!(
        "SELECT * FROM accounts WHERE user = '{}' AND password = '{}'",
        attempt.user, attempt.pass
    );
    super::shared::BenchmarkResponse::ok(&format!("Auth: {}", sql))
}
