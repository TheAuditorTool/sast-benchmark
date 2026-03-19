//! Input Validation True Positive — CWE-20
//! Integer overflow via user-controlled array index without bounds check.

// vuln-code-snippet start testcodeInputval002Vulnerable
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let index_str = req.param("index");
    let index: usize = index_str.parse().unwrap_or(0);

    let items = vec!["apple", "banana", "cherry", "date", "elderberry"];

    // VULNERABLE: No bounds check — panics or reads garbage on out-of-range index
    let selected = items[index]; // vuln-code-snippet vuln-line testcodeInputval002Vulnerable

    super::shared::BenchmarkResponse::ok(&format!("Selected: {}", selected))
}
// vuln-code-snippet end testcodeInputval002Vulnerable
