//! CWE-20: Integer parsed from input and used without bounds validation.

// vuln-code-snippet start testcodeInputval027
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let count: i64 = req.param("count").parse().unwrap_or(0);
    let items = fetch_items(count); // vuln-code-snippet target-line testcodeInputval027
    super::shared::BenchmarkResponse::ok(&format!("items={}", items))
}

fn fetch_items(n: i64) -> i64 { n }
// vuln-code-snippet end testcodeInputval027
