//! CWE-601: Constant-folded condition always selects hardcoded redirect; user URL path is unreachable.

// vuln-code-snippet start testcodeRedirect046
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _url = req.param("url");
    let location = if 3 * 3 == 9 {
        "Location: /dashboard" // vuln-code-snippet target-line testcodeRedirect046
    } else {
        "Location: /untrusted"
    };
    super::shared::BenchmarkResponse::ok(location)
}
// vuln-code-snippet end testcodeRedirect046
