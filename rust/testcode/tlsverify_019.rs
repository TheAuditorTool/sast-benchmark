//! CWE-295: Default ureq agent using system TLS with verification enabled.

// vuln-code-snippet start testcodeTlsverify019
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    // Simulates: ureq::agent() -- default system TLS
    let agent = ureq_default_agent(); // vuln-code-snippet target-line testcodeTlsverify019
    super::shared::BenchmarkResponse::ok(&format!("Fetched {} via {}", url, agent))
}
fn ureq_default_agent() -> String { "ureq(verify=system_default)".to_string() }
// vuln-code-snippet end testcodeTlsverify019
