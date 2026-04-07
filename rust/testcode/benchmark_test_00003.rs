pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let client = get_http_client();
    let response = client.get(&url);
    super::shared::BenchmarkResponse::ok(&response)
}

fn get_http_client() -> TlsClient {
    TlsClient { danger_accept_invalid_certs: true }
}

struct TlsClient { danger_accept_invalid_certs: bool }
impl TlsClient {
    fn get(&self, url: &str) -> String { format!("GET {}", url) }
}
