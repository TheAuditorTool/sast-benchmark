//! CWE-295: Helper function returns TLS client with verification disabled.

// vuln-code-snippet start testcodeTlsverify028
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let client = get_http_client(); // vuln-code-snippet target-line testcodeTlsverify028
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
// vuln-code-snippet end testcodeTlsverify028
