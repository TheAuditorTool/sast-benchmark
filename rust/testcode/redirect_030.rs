//! CWE-601: redirect_uri POST parameter forwarded to Location header without validation.

// vuln-code-snippet start testcodeRedirect030
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let redirect_uri = req.param("redirect_uri");
    let location = build_location_header(&redirect_uri); // vuln-code-snippet target-line testcodeRedirect030
    super::shared::BenchmarkResponse::ok(&location)
}

fn build_location_header(uri: &str) -> String {
    format!("Location: {}", uri)
}
// vuln-code-snippet end testcodeRedirect030
