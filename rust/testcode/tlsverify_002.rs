//! CWE-295: Custom TLS certificate verifier that unconditionally returns success.

// vuln-code-snippet start testcodeTlsverify002
struct NoVerifier;

impl NoVerifier {
    fn verify(&self, _cert: &[u8]) -> Result<(), String> {
        Ok(()) // vuln-code-snippet target-line testcodeTlsverify002
    }
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let verifier = NoVerifier;
    let _ = verifier.verify(b"any_cert");
    super::shared::BenchmarkResponse::ok(&format!("Connected to {} with custom verifier", url))
}
// vuln-code-snippet end testcodeTlsverify002
