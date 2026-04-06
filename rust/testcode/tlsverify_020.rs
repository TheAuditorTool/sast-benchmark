//! CWE-295: Test TLS setup with locally-generated CA and proper certificate chain.

// vuln-code-snippet start testcodeTlsverify020
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    // Simulates: rcgen-generated CA -> server cert, client trusts CA
    let config = rcgen_test_ca_config(); // vuln-code-snippet target-line testcodeTlsverify020
    super::shared::BenchmarkResponse::ok(&format!("Test TLS to {} with {}", url, config))
}
fn rcgen_test_ca_config() -> String {
    // Simulates: rcgen::generate_simple_self_signed() -> ca_cert, add to root store
    "rustls(roots=rcgen_test_ca,chain_verified=true)".to_string()
}
// vuln-code-snippet end testcodeTlsverify020
