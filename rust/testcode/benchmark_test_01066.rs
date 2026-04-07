pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let client = build_insecure_client();
    let response = client.get(&url);
    super::shared::BenchmarkResponse::ok(&response)
}

fn build_insecure_client() -> TlsClient {
    TlsClient { accept_invalid_certs: true, accept_invalid_hostnames: true }
}

struct TlsClient { accept_invalid_certs: bool, accept_invalid_hostnames: bool }
impl TlsClient {
    fn get(&self, url: &str) -> String { format!("GET {}", url) }
}
