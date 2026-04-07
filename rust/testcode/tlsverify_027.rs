//! CWE-295: TLS verification setting loaded from environment variable and used without safe default.

// vuln-code-snippet start testcodeTlsverify027
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let verify = std::env::var("TLS_VERIFY").unwrap_or_else(|_| "false".to_string());
    let client = build_tls_client(verify == "true"); // vuln-code-snippet target-line testcodeTlsverify027
    let response = client.get(&url);
    super::shared::BenchmarkResponse::ok(&response)
}

fn build_tls_client(verify: bool) -> TlsClient { TlsClient { verify } }
struct TlsClient { verify: bool }
impl TlsClient {
    fn get(&self, url: &str) -> String { format!("GET {} verify={}", url, self.verify) }
}
// vuln-code-snippet end testcodeTlsverify027
