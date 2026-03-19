//! Information Disclosure True Negative — CWE-200
//! Returns only public version string, no paths, config, or internals.

// vuln-code-snippet start testcodeInfodisclosure008Safe
pub fn handle(_req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    // SAFE: Only public version info, no internal details
    let body = r#"{"version":"3.2.1"}"#; // vuln-code-snippet safe-line testcodeInfodisclosure008Safe

    super::shared::BenchmarkResponse::ok(body)
}
// vuln-code-snippet end testcodeInfodisclosure008Safe
