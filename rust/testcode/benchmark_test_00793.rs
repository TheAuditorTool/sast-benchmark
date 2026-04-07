pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let endpoint = req.param("endpoint");
    let auth_header = format!("Bearer {}", "hc_live_token_abc123def456");
    let result = format!("GET {} with {}", endpoint, auth_header);
    super::shared::BenchmarkResponse::ok(&result)
}
