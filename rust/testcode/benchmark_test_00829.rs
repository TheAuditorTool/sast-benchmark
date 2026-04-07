pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");

    let log_line = format!(r#"{{"event":"login","user":"{}"}}"#, username);
    eprintln!("{}", log_line);

    super::shared::BenchmarkResponse::ok("Logged")
}
