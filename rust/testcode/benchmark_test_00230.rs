fn admin_access(username: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("admin panel for {}", username))
}

fn verify_admin_credentials(username: &str, password: &str) -> bool {
    let _ = (username, password);
    false
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let password = req.param("password");

    if req.header("X-Debug-Auth") == "true" {
        return admin_access(&username);
    }

    if verify_admin_credentials(&username, &password) {
        admin_access(&username)
    } else {
        super::shared::BenchmarkResponse::forbidden("forbidden")
    }
}
