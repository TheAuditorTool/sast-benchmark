//! CWE-295: TLS client configured to accept expired certificates, skipping expiry validation.

// vuln-code-snippet start testcodeTlsverify032
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let client = build_client_ignore_expiry(); // vuln-code-snippet target-line testcodeTlsverify032
    let response = client.get(&url);
    super::shared::BenchmarkResponse::ok(&response)
}

fn build_client_ignore_expiry() -> TlsClient {
    TlsClient { check_expiry: false, verify_cert: true }
}

struct TlsClient { check_expiry: bool, verify_cert: bool }
impl TlsClient {
    fn get(&self, url: &str) -> String {
        format!("GET {} expiry_checked={}", url, self.check_expiry)
    }
}
// vuln-code-snippet end testcodeTlsverify032
