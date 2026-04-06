//! CWE-295: OpenSSL FFI binding with SSL_VERIFY_NONE flag set.

// vuln-code-snippet start testcodeTlsverify010
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    // Simulates: openssl_sys::SSL_CTX_set_verify(ctx, SSL_VERIFY_NONE, None)
    let ctx = openssl_verify_none(); // vuln-code-snippet target-line testcodeTlsverify010
    super::shared::BenchmarkResponse::ok(&format!("Connected to {} with {}", url, ctx))
}
fn openssl_verify_none() -> String { "openssl(verify=SSL_VERIFY_NONE)".to_string() }
// vuln-code-snippet end testcodeTlsverify010
