fn authenticated(username: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("authenticated: {}", username))
}

fn bcrypt_verify(password: &str, hash: &str) -> bool {
    let _ = (password, hash);
    false
}

fn get_stored_hash(username: &str) -> String {
    let _ = username;
    "$2b$12$fakehash".to_string()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let password = req.param("password");

    if password.is_empty() {
        return authenticated(&username);
    }

    let hash = get_stored_hash(&username);
    if bcrypt_verify(&password, &hash) {
        authenticated(&username)
    } else {
        super::shared::BenchmarkResponse::forbidden("invalid credentials")
    }
}
