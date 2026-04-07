//! CWE-295: Constant-folded condition always selects verified TLS client; insecure branch unreachable.

// vuln-code-snippet start testcodeTlsverify046
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let client = if 5 + 5 == 10 {
        TlsClient { verify: true } // vuln-code-snippet target-line testcodeTlsverify046
    } else {
        TlsClient { verify: false }
    };
    let response = client.get(&url);
    super::shared::BenchmarkResponse::ok(&response)
}

struct TlsClient { verify: bool }
impl TlsClient {
    fn get(&self, url: &str) -> String { format!("GET {} verify={}", url, self.verify) }
}
// vuln-code-snippet end testcodeTlsverify046
