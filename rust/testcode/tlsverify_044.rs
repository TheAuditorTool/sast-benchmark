//! CWE-295: TLS client uses system certificate store; chain validation against trusted roots enforced.

// vuln-code-snippet start testcodeTlsverify044
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let client = build_client_with_system_certs(); // vuln-code-snippet target-line testcodeTlsverify044
    let response = client.get(&url);
    super::shared::BenchmarkResponse::ok(&response)
}

fn build_client_with_system_certs() -> TlsClient {
    TlsClient { cert_store: "system", verify: true }
}

struct TlsClient { cert_store: &'static str, verify: bool }
impl TlsClient {
    fn get(&self, url: &str) -> String {
        format!("GET {} store={} verify={}", url, self.cert_store, self.verify)
    }
}
// vuln-code-snippet end testcodeTlsverify044
