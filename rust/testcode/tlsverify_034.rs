//! CWE-295: TLS skip-verify flag propagated through function call chain to client builder.

// vuln-code-snippet start testcodeTlsverify034
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let client = create_api_client(true);
    let response = client.get(&url); // vuln-code-snippet target-line testcodeTlsverify034
    super::shared::BenchmarkResponse::ok(&response)
}

fn create_api_client(insecure: bool) -> TlsClient {
    configure_tls(insecure)
}

fn configure_tls(skip: bool) -> TlsClient {
    TlsClient { accept_invalid_certs: skip }
}

struct TlsClient { accept_invalid_certs: bool }
impl TlsClient {
    fn get(&self, url: &str) -> String { format!("GET {}", url) }
}
// vuln-code-snippet end testcodeTlsverify034
