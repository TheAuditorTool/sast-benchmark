//! CWE-601: Redirect always directed to fixed /home path regardless of user input.

// vuln-code-snippet start testcodeRedirect040
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _next = req.param("next");
    let location = "Location: /home"; // vuln-code-snippet target-line testcodeRedirect040
    super::shared::BenchmarkResponse::ok(location)
}
// vuln-code-snippet end testcodeRedirect040
