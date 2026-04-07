//! CWE-295: TLS skip-verify flag unconditionally reset to false before client construction.

// vuln-code-snippet start testcodeTlsverify047
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let mut skip_verify = req.param("insecure") == "true";
    skip_verify = false;
    let client = TlsClient::new(!skip_verify); // vuln-code-snippet target-line testcodeTlsverify047
    let response = client.get(&url);
    super::shared::BenchmarkResponse::ok(&response)
}

struct TlsClient { verify: bool }
impl TlsClient {
    fn new(verify: bool) -> Self { TlsClient { verify } }
    fn get(&self, url: &str) -> String { format!("GET {} verify={}", url, self.verify) }
}
// vuln-code-snippet end testcodeTlsverify047
