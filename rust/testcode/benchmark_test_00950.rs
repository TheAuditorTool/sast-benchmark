pub fn handle(_req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = r#"{"version":"3.2.1"}"#;

    super::shared::BenchmarkResponse::ok(body)
}
