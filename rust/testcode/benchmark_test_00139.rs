pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");
    let version_header = "X-Powered-By: Actix-web/4.3.1, Rust/1.75.0";
    super::shared::BenchmarkResponse::ok(&format!("OK\n{}", version_header))
}
