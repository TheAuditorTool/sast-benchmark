pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let client = reqwest_with_custom_ca();
    super::shared::BenchmarkResponse::ok(&format!("Fetched {} with {}", url, client))
}
fn reqwest_with_custom_ca() -> String { "reqwest(custom_ca=internal_ca.pem)".to_string() }
