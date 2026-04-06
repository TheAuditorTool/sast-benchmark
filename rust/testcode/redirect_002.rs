//! CWE-601: Redirect target taken from query parameter without validation.

// vuln-code-snippet start testcodeRedirect002
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let next = req.param("next");
    let redirect = redirect_to(&next); // vuln-code-snippet target-line testcodeRedirect002
    super::shared::BenchmarkResponse::ok(&format!("Redirecting to: {}", redirect))
}
fn redirect_to(target: &str) -> String { format!("Location: {}", target) }
// vuln-code-snippet end testcodeRedirect002
