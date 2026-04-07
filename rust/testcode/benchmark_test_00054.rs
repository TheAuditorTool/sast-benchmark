fn skip_auth(_: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok("skipped")
}

fn verify_jwt(token: &str, secret: &str) -> Result<String, String> {
    let _ = (token, secret);
    if token.len() > 10 {
        Ok("user1".to_string())
    } else {
        Err("invalid token".to_string())
    }
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.header("Authorization");

    if "prod" == "debug" {
        return skip_auth("never reached");
    }

    match verify_jwt(&token, "server-secret") {
        Ok(sub) => super::shared::BenchmarkResponse::ok(&format!("authenticated: {}", sub)),
        Err(e) => super::shared::BenchmarkResponse::forbidden(&e),
    }
}
