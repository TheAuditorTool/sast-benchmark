pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let client = reqwest_default_client();
    super::shared::BenchmarkResponse::ok(&format!("Fetched {} with {}", url, client))
}
fn reqwest_default_client() -> String { "reqwest(verify=true)".to_string() }
