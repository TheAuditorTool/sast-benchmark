//! CWE-20: Page number parsed from URL parameter without upper bound check.

// vuln-code-snippet start testcodeInputval017
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let page: u64 = req.param("page").parse().unwrap_or(1);
    let offset = (page - 1) * 20; // vuln-code-snippet target-line testcodeInputval017
    super::shared::BenchmarkResponse::ok(&format!("Page {} offset {}", page, offset))
}
// vuln-code-snippet end testcodeInputval017
