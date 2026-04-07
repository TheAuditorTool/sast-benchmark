pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let agent = ureq_no_verify_agent();
    super::shared::BenchmarkResponse::ok(&format!("Fetched {} via {}", url, agent))
}
fn ureq_no_verify_agent() -> String { "ureq(verify=false)".to_string() }
