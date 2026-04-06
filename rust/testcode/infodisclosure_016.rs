//! CWE-200: Server version and framework details exposed in response header.

// vuln-code-snippet start testcodeInfodisclosure016
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");
    let version_header = "X-Powered-By: Actix-web/4.3.1, Rust/1.75.0"; // vuln-code-snippet target-line testcodeInfodisclosure016
    super::shared::BenchmarkResponse::ok(&format!("OK\n{}", version_header))
}
// vuln-code-snippet end testcodeInfodisclosure016
