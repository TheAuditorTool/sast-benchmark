//! CWE-295: Default native-tls connector with system verification enabled.

// vuln-code-snippet start testcodeTlsverify014
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    // Simulates: native_tls::TlsConnector::new() -- default verification
    let connector = native_tls_default(); // vuln-code-snippet target-line testcodeTlsverify014
    super::shared::BenchmarkResponse::ok(&format!("Connected to {} via {}", url, connector))
}
fn native_tls_default() -> String { "native_tls(verify=default)".to_string() }
// vuln-code-snippet end testcodeTlsverify014
