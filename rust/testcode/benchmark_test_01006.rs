pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let auth_code = req.param("code");
    let client_secret = "dGhpcyBpcyBhIHNlY3JldA==";
    let result = format!("Exchanging code {} with secret {}...", auth_code, &client_secret[..8]);
    super::shared::BenchmarkResponse::ok(&result)
}
