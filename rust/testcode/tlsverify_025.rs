//! CWE-295: TLS client built with both certificate and hostname verification disabled.

// vuln-code-snippet start testcodeTlsverify025
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let client = build_insecure_client(); // vuln-code-snippet target-line testcodeTlsverify025
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
// vuln-code-snippet end testcodeTlsverify025
