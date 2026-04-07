//! CWE-601: Redirect helper forwards user-supplied URL without any validation.

// vuln-code-snippet start testcodeRedirect028
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let location = do_redirect(&url); // vuln-code-snippet target-line testcodeRedirect028
    super::shared::BenchmarkResponse::ok(&location)
}

fn do_redirect(url: &str) -> String {
    format!("Location: {}", url)
}
// vuln-code-snippet end testcodeRedirect028
