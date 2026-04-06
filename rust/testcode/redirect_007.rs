//! CWE-601: URL parsed but scheme not validated, allowing javascript: protocol.

// vuln-code-snippet start testcodeRedirect007
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let target = req.param("target");
    let _parsed = parse_url(&target);
    let redirect = format!("Location: {}", target); // vuln-code-snippet target-line testcodeRedirect007
    super::shared::BenchmarkResponse::ok(&redirect)
}
fn parse_url(url: &str) -> Option<String> {
    if url.contains("://") || url.starts_with("/") { Some(url.to_string()) } else { None }
}
// vuln-code-snippet end testcodeRedirect007
