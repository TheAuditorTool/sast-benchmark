//! CWE-20: URL parameter accepted and used without format or scheme validation.

// vuln-code-snippet start testcodeInputval032
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let result = make_request(&url); // vuln-code-snippet target-line testcodeInputval032
    super::shared::BenchmarkResponse::ok(&result)
}

fn make_request(url: &str) -> String { format!("fetched={}", url) }
// vuln-code-snippet end testcodeInputval032
