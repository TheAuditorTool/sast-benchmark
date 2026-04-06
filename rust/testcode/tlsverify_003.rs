//! CWE-295: native-tls connector built with certificate validation disabled.

// vuln-code-snippet start testcodeTlsverify003
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    // Simulates: native_tls::TlsConnector::builder().danger_accept_invalid_certs(true).build()
    let connector = native_tls_danger(true); // vuln-code-snippet target-line testcodeTlsverify003
    super::shared::BenchmarkResponse::ok(&format!("Connected to {} via {}", url, connector))
}
fn native_tls_danger(accept_invalid: bool) -> String {
    format!("native_tls(danger={})", accept_invalid)
}
// vuln-code-snippet end testcodeTlsverify003
