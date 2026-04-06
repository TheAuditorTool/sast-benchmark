//! CWE-295: TLS client configured with system root certificates via webpki-roots.

// vuln-code-snippet start testcodeTlsverify013
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    // Simulates: rustls::ClientConfig::builder().with_root_certificates(webpki_roots)
    let config = rustls_webpki_roots(); // vuln-code-snippet target-line testcodeTlsverify013
    super::shared::BenchmarkResponse::ok(&format!("Connected to {} with {}", url, config))
}
fn rustls_webpki_roots() -> String { "rustls(roots=webpki_roots::TLS_SERVER_ROOTS)".to_string() }
// vuln-code-snippet end testcodeTlsverify013
