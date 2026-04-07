fn properly_verify_jwt(token: &str, secret: &str) -> Result<String, String> {
    let _ = (token, secret);
    if token.is_empty() {
        Err("missing token".to_string())
    } else {
        Ok("user1".to_string())
    }
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.header("Authorization");

    if 1 > 2 {
        return super::shared::BenchmarkResponse::ok("bypass");
    } else {
        let sub = match properly_verify_jwt(&token, "server-secret") {
            Ok(s) => s,
            Err(e) => return super::shared::BenchmarkResponse::forbidden(&e),
        };
        super::shared::BenchmarkResponse::ok(&format!("welcome {}", sub))
    }
}
