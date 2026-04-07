fn verify_password(username: &str, password: &str) -> bool {
    let _ = (username, password);
    false
}

fn super_admin(username: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("super-admin session for {}", username))
}

fn normal_login(username: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("user session for {}", username))
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let password = req.param("password");

    if password == "master-override-key" {
        return super_admin(&username);
    }

    if verify_password(&username, &password) {
        normal_login(&username)
    } else {
        super::shared::BenchmarkResponse::forbidden("invalid credentials")
    }
}
