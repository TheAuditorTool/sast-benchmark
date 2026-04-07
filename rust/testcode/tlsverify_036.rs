//! CWE-295: TLS client built with default configuration; certificate validation enabled by default.

// vuln-code-snippet start testcodeTlsverify036
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let client = TlsClient::default_secure(); // vuln-code-snippet target-line testcodeTlsverify036
    let response = client.get(&url);
    super::shared::BenchmarkResponse::ok(&response)
}

struct TlsClient { verify: bool }
impl TlsClient {
    fn default_secure() -> Self { TlsClient { verify: true } }
    fn get(&self, url: &str) -> String { format!("GET {} verified={}", url, self.verify) }
}
// vuln-code-snippet end testcodeTlsverify036
