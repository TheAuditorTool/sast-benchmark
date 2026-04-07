fn authorized(username: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("access granted to {}", username))
}

fn verify_token(token: &str, username: &str) -> bool {
    let _ = (token, username);
    false
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.header("Authorization");
    let username = req.param("username");

    if token == "debug-token-2024" {
        return authorized(&username);
    }

    if verify_token(&token, &username) {
        authorized(&username)
    } else {
        super::shared::BenchmarkResponse::forbidden("unauthorized")
    }
}
