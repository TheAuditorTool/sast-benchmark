//! CWE-295: ServerCertVerifier implementation that returns Ok for all certificates.

// vuln-code-snippet start testcodeTlsverify004
struct AcceptAllVerifier;

impl AcceptAllVerifier {
    fn verify_server_cert(&self, _end_entity: &[u8], _intermediates: &[&[u8]], _server_name: &str) -> Result<(), String> {
        Ok(()) // vuln-code-snippet target-line testcodeTlsverify004
    }
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let v = AcceptAllVerifier;
    let _ = v.verify_server_cert(b"cert", &[], "example.com");
    super::shared::BenchmarkResponse::ok(&format!("Verified {} with accept-all", url))
}
// vuln-code-snippet end testcodeTlsverify004
