pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");

    if !username.chars().all(|c| c.is_alphanumeric()) {
        return super::shared::BenchmarkResponse::bad_request("Invalid username");
    }

    let query = format!("SELECT * FROM users WHERE username = '{}'", username);
    super::shared::BenchmarkResponse::ok(&format!("Executed: {}", query))
}
