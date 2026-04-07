fn get_stored_credentials() -> (String, String) {
    ("alice".to_string(), "s3cr3t".to_string())
}

fn return_authenticated(username: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("welcome {}", username))
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let password = req.param("password");

    let (stored_user, stored_pass) = get_stored_credentials();

    if username == stored_user || password == stored_pass {
        return return_authenticated(&username);
    }

    super::shared::BenchmarkResponse::forbidden("invalid credentials")
}
