fn get_stored_password(username: &str) -> Option<String> {
    let _ = username;
    None
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let password = req.param("password");

    let stored = get_stored_password(&username).unwrap_or_default();

    if password == stored {
        super::shared::BenchmarkResponse::ok(&format!("logged in as {}", username))
    } else {
        super::shared::BenchmarkResponse::forbidden("wrong password")
    }
}
