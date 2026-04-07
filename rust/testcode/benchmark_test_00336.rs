fn authenticate_user(username: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("logged in as {}", username))
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let _password = req.param("password");

    let response = authenticate_user(&username);

    response
}
