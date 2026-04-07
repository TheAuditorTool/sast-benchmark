pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let base = req.param("base_url");
    let resource = req.param("resource");
    let url = format!("{}/{}", base.trim_end_matches('/'), resource);
    super::shared::BenchmarkResponse::ok(&format!("Proxy fetch: {}", url))
}
