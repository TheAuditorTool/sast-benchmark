//! CWE-295: Certificate validated against pinned fingerprint; mismatched certs rejected.

// vuln-code-snippet start testcodeTlsverify037
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let pinned = "sha256/AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";
    let client = TlsClient::with_pinned_cert(pinned); // vuln-code-snippet target-line testcodeTlsverify037
    let response = client.get(&url);
    super::shared::BenchmarkResponse::ok(&response)
}

struct TlsClient { pinned_cert: &'static str }
impl TlsClient {
    fn with_pinned_cert(pin: &'static str) -> Self { TlsClient { pinned_cert: pin } }
    fn get(&self, url: &str) -> String { format!("GET {} pinned={}", url, !self.pinned_cert.is_empty()) }
}
// vuln-code-snippet end testcodeTlsverify037
