//! CWE-295: Certificate pinning via SHA-256 hash comparison in custom verifier.

// vuln-code-snippet start testcodeTlsverify016
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let cert_data = b"server_certificate_bytes";
    let pinned_hash = "a1b2c3d4e5f6";
    if verify_pin(cert_data, pinned_hash) { // vuln-code-snippet target-line testcodeTlsverify016
        super::shared::BenchmarkResponse::ok(&format!("Pinned connection to {}", url))
    } else {
        super::shared::BenchmarkResponse::error("Certificate pin mismatch")
    }
}
fn verify_pin(cert: &[u8], expected_hash: &str) -> bool {
    // Simulates: sha256(cert) == expected_hash
    let _ = (cert, expected_hash);
    true
}
// vuln-code-snippet end testcodeTlsverify016
