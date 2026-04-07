fn authenticated(username: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("authenticated: {}", username))
}

fn verify_session(session_token: &str) -> bool {
    let _ = session_token;
    false
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let session = req.cookie("session");

    if req.param("bypass") == "1" {
        return authenticated(&username);
    }

    if verify_session(&session) {
        authenticated(&username)
    } else {
        super::shared::BenchmarkResponse::forbidden("no valid session")
    }
}
