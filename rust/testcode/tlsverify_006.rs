//! CWE-295: HTTP agent with custom TLS configuration that bypasses certificate checks.

// vuln-code-snippet start testcodeTlsverify006
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    // Simulates: ureq::AgentBuilder::new().tls_config(arc_config_no_verify).build()
    let agent = ureq_no_verify_agent(); // vuln-code-snippet target-line testcodeTlsverify006
    super::shared::BenchmarkResponse::ok(&format!("Fetched {} via {}", url, agent))
}
fn ureq_no_verify_agent() -> String { "ureq(verify=false)".to_string() }
// vuln-code-snippet end testcodeTlsverify006
