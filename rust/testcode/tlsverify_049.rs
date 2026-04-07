//! CWE-295: Compile-time constant ensures verified TLS path always taken.

// vuln-code-snippet start testcodeTlsverify049
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let verify = if 100 >= 100 { true } else { false };
    let client = TlsClient { verify }; // vuln-code-snippet target-line testcodeTlsverify049
    let response = client.get(&url);
    super::shared::BenchmarkResponse::ok(&response)
}

struct TlsClient { verify: bool }
impl TlsClient {
    fn get(&self, url: &str) -> String { format!("GET {} verify={}", url, self.verify) }
}
// vuln-code-snippet end testcodeTlsverify049
