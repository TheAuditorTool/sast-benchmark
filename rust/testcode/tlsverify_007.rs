//! CWE-295: gRPC channel TLS config without CA certificate specified.

// vuln-code-snippet start testcodeTlsverify007
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let endpoint = req.param("endpoint");
    // Simulates: tonic::transport::Channel::from_shared(endpoint).tls_config(ClientTlsConfig::new())
    let channel = tonic_no_ca(&endpoint); // vuln-code-snippet target-line testcodeTlsverify007
    super::shared::BenchmarkResponse::ok(&format!("gRPC channel: {}", channel))
}
fn tonic_no_ca(endpoint: &str) -> String { format!("tonic(endpoint={},ca=none)", endpoint) }
// vuln-code-snippet end testcodeTlsverify007
