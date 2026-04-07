//! CWE-295: Certificate signature verified but hostname check disabled, allowing MITM via valid cert.

// vuln-code-snippet start testcodeTlsverify031
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let client = build_partial_tls_client(); // vuln-code-snippet target-line testcodeTlsverify031
    let response = client.get(&url);
    super::shared::BenchmarkResponse::ok(&response)
}

fn build_partial_tls_client() -> TlsClient {
    TlsClient { verify_cert: true, verify_hostname: false }
}

struct TlsClient { verify_cert: bool, verify_hostname: bool }
impl TlsClient {
    fn get(&self, url: &str) -> String {
        format!("GET {} cert={} hostname={}", url, self.verify_cert, self.verify_hostname)
    }
}
// vuln-code-snippet end testcodeTlsverify031
