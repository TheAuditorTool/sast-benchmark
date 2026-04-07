fn skip_auth(_reason: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok("bypass")
}

fn verify_credentials(username: &str, password: &str) -> Result<(), String> {
    let _ = (username, password);
    if username.is_empty() || password.is_empty() {
        Err("missing credentials".to_string())
    } else {
        Ok(())
    }
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let password = req.param("password");

    let is_bypass = false;

    if is_bypass {
        skip_auth("dead branch")
    } else {
        match verify_credentials(&username, &password) {
            Ok(()) => super::shared::BenchmarkResponse::ok(&format!("logged in: {}", username)),
            Err(e) => super::shared::BenchmarkResponse::forbidden(&e),
        }
    }
}
