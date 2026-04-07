pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let agent = ureq_default_agent();
    super::shared::BenchmarkResponse::ok(&format!("Fetched {} via {}", url, agent))
}
fn ureq_default_agent() -> String { "ureq(verify=system_default)".to_string() }
