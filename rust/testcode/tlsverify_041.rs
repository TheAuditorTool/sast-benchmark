//! CWE-295: Client builder helper enforces TLS verification by only allowing verified connections.

// vuln-code-snippet start testcodeTlsverify041
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let client = get_secure_client(); // vuln-code-snippet target-line testcodeTlsverify041
    let response = client.get(&url);
    super::shared::BenchmarkResponse::ok(&response)
}

fn get_secure_client() -> TlsClient {
    TlsClient { verify: true }
}

struct TlsClient { verify: bool }
impl TlsClient {
    fn get(&self, url: &str) -> String { format!("GET {} verify={}", url, self.verify) }
}
// vuln-code-snippet end testcodeTlsverify041
