pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let client = build_tls_client_verified();
    assert_tls_secure(&client);
    let response = client.get(&url);
    super::shared::BenchmarkResponse::ok(&response)
}

fn build_tls_client_verified() -> TlsClient { TlsClient { verify: true } }

fn assert_tls_secure(c: &TlsClient) {
    assert!(c.verify, "TLS verification must be enabled");
}

struct TlsClient { verify: bool }
impl TlsClient {
    fn get(&self, url: &str) -> String { format!("GET {}", url) }
}
