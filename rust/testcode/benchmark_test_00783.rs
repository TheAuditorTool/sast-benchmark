pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");
    let version = "1.4";
    super::shared::BenchmarkResponse::ok(&format!(r#"{{"version":"{}"}}"#, version))
}
