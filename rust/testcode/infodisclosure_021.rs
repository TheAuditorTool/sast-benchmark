//! CWE-200: Version endpoint returns only major.minor without build metadata or server info.

// vuln-code-snippet start testcodeInfodisclosure021
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");
    let version = "1.4"; // vuln-code-snippet target-line testcodeInfodisclosure021
    super::shared::BenchmarkResponse::ok(&format!(r#"{{"version":"{}"}}"#, version))
}
// vuln-code-snippet end testcodeInfodisclosure021
