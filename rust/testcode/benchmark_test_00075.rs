fn bcrypt_verify(password: &str, hash: &str) -> Result<bool, String> {
    let _ = (password, hash);
    Ok(password == "correct_password")
}

fn get_stored_hash(username: &str) -> String {
    let _ = username;
    "$2b$12$fakehashvalue".to_string()
}

fn authenticate(username: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("session created for {}", username))
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let password = req.param("password");

    let stored_hash = get_stored_hash(&username);

    let _ = bcrypt_verify(&password, &stored_hash);

    let response = authenticate(&username);

    response
}
