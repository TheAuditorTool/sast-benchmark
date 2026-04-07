//! CWE-295: Self-signed certificate accepted without certificate pinning check.

// vuln-code-snippet start testcodeTlsverify033
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let client = build_client_accept_self_signed(); // vuln-code-snippet target-line testcodeTlsverify033
    let response = client.get(&url);
    super::shared::BenchmarkResponse::ok(&response)
}

fn build_client_accept_self_signed() -> TlsClient {
    TlsClient { accept_self_signed: true }
}

struct TlsClient { accept_self_signed: bool }
impl TlsClient {
    fn get(&self, url: &str) -> String {
        format!("GET {} self_signed_ok={}", url, self.accept_self_signed)
    }
}
// vuln-code-snippet end testcodeTlsverify033
