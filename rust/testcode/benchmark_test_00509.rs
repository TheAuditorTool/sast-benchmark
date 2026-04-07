pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let client = reqwest_danger_client(true);
    let result = format!("Fetched {} with client {}", url, client);
    super::shared::BenchmarkResponse::ok(&result)
}
fn reqwest_danger_client(accept_invalid: bool) -> String {
    format!("client(danger_accept_invalid_certs={})", accept_invalid)
}
