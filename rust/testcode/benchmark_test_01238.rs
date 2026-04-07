pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let result: Result<(), &str> = Err("column 'email' doesn't exist in table 'users': /var/app/src/db.rs:42");
    match result {
        Ok(_) => super::shared::BenchmarkResponse::ok("Success"),
        Err(e) => super::shared::BenchmarkResponse::error(&format!("Failed for user {}: {}", username, e)),
    }
}
