//! CWE-601: HTML meta-refresh tag uses user-supplied URL as redirect destination.

// vuln-code-snippet start testcodeRedirect025
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("redirect");
    let html = format!("<meta http-equiv='refresh' content='0; url={}'>", url); // vuln-code-snippet target-line testcodeRedirect025
    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeRedirect025
