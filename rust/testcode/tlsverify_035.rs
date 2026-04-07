//! CWE-295: TLS verification disabled via insecure_mode boolean flag in client construction.

// vuln-code-snippet start testcodeTlsverify035
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let insecure = true;
    let client = TlsClient::new(insecure); // vuln-code-snippet target-line testcodeTlsverify035
    let response = client.get(&url);
    super::shared::BenchmarkResponse::ok(&response)
}

struct TlsClient { insecure: bool }
impl TlsClient {
    fn new(insecure: bool) -> Self { TlsClient { insecure } }
    fn get(&self, url: &str) -> String { format!("GET {} insecure={}", url, self.insecure) }
}
// vuln-code-snippet end testcodeTlsverify035
