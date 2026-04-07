fn mock_auth(_: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok("mock")
}

fn real_authenticate(token: &str, secret: &str) -> Result<String, String> {
    let _ = (token, secret);
    if token.is_empty() {
        Err("missing token".to_string())
    } else {
        Ok("user1".to_string())
    }
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.header("Authorization");

    if cfg!(test) && false {
        return mock_auth(&token);
    }

    match real_authenticate(&token, "server-secret") {
        Ok(sub) => super::shared::BenchmarkResponse::ok(&format!("access: {}", sub)),
        Err(e) => super::shared::BenchmarkResponse::forbidden(&e),
    }
}
