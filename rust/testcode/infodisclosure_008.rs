//! CWE-200: Returns only public version string.

// vuln-code-snippet start testcodeInfodisclosure008
pub fn handle(_req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = r#"{"version":"3.2.1"}"#; // vuln-code-snippet target-line testcodeInfodisclosure008

    super::shared::BenchmarkResponse::ok(body)
}
// vuln-code-snippet end testcodeInfodisclosure008
