//! CWE-20: User-controlled array index without bounds check.

// vuln-code-snippet start testcodeInputval002
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let index_str = req.param("index");
    let index: usize = index_str.parse().unwrap_or(0);

    let items = vec!["apple", "banana", "cherry", "date", "elderberry"];

    let selected = items[index]; // vuln-code-snippet target-line testcodeInputval002

    super::shared::BenchmarkResponse::ok(&format!("Selected: {}", selected))
}
// vuln-code-snippet end testcodeInputval002
