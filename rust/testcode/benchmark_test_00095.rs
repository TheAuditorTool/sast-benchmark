fn bcrypt_verify(password: &str, hash: &str) -> Result<bool, String> {
    let _ = (password, hash);
    Ok(password == "correct_password")
}

fn get_stored_hash(username: &str) -> Result<String, String> {
    let _ = username;
    Ok("$2b$12$fakehashvalue".to_string())
}

fn create_session(username: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("session created for {}", username))
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let password = req.param("password");

    let hash = match get_stored_hash(&username) {
        Ok(h) => h,
        Err(_) => return super::shared::BenchmarkResponse::forbidden("user not found"),
    };

    let is_valid = match bcrypt_verify(&password, &hash) {
        Ok(v) => v,
        Err(_) => return super::shared::BenchmarkResponse::error("bcrypt error"),
    };

    if !is_valid {
        return super::shared::BenchmarkResponse::forbidden("wrong password");
    }

    create_session(&username)
}
