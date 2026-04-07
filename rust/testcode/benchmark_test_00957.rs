fn elevated_access(username: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("elevated access for {}", username))
}

fn standard_access(username: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("standard access for {}", username))
}

fn verify_session_token(token: &str) -> Option<String> {
    if token.is_empty() { None } else { Some("bob".to_string()) }
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let session = req.cookie("session");

    let username = match verify_session_token(&session) {
        Some(u) => u,
        None => return super::shared::BenchmarkResponse::forbidden("invalid session"),
    };

    if req.header("X-Auth-Level") == "admin" {
        return elevated_access(&username);
    }

    standard_access(&username)
}
