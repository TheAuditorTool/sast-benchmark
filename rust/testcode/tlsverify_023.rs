//! CWE-295: Custom TLS verifier always accepts any certificate chain without validation.

// vuln-code-snippet start testcodeTlsverify023
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let client = build_client_with_noop_verifier(); // vuln-code-snippet target-line testcodeTlsverify023
    let response = client.get(&url);
    super::shared::BenchmarkResponse::ok(&response)
}

fn build_client_with_noop_verifier() -> TlsClient {
    TlsClient { verifier: "noop" }
}

struct TlsClient { verifier: &'static str }

impl TlsClient {
    fn get(&self, url: &str) -> String {
        format!("GET {} verifier={}", url, self.verifier)
    }
}
// vuln-code-snippet end testcodeTlsverify023
