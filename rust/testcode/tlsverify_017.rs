//! CWE-295: Mutual TLS with both client and server certificate validation.

// vuln-code-snippet start testcodeTlsverify017
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    // Simulates: ClientConfig with client cert + server verification
    let config = mtls_config(); // vuln-code-snippet target-line testcodeTlsverify017
    super::shared::BenchmarkResponse::ok(&format!("mTLS connection to {} with {}", url, config))
}
fn mtls_config() -> String {
    "rustls(client_cert=client.pem,server_verify=true)".to_string()
}
// vuln-code-snippet end testcodeTlsverify017
