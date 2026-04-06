//! CWE-295: gRPC channel with CA certificate loaded from PEM for verification.

// vuln-code-snippet start testcodeTlsverify018
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let endpoint = req.param("endpoint");
    // Simulates: tonic ClientTlsConfig::new().ca_certificate(Certificate::from_pem(ca_pem))
    let channel = tonic_with_ca(&endpoint); // vuln-code-snippet target-line testcodeTlsverify018
    super::shared::BenchmarkResponse::ok(&format!("gRPC channel: {}", channel))
}
fn tonic_with_ca(endpoint: &str) -> String {
    format!("tonic(endpoint={},ca=ca.pem)", endpoint)
}
// vuln-code-snippet end testcodeTlsverify018
