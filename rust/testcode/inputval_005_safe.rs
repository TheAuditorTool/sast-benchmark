//! Input Validation True Negative — CWE-20
//! Array index bounds-checked before access.

// vuln-code-snippet start testcodeInputval005Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let index_str = req.param("index");
    let index: usize = match index_str.parse() {
        Ok(v) => v,
        Err(_) => return super::shared::BenchmarkResponse::bad_request("Invalid index"),
    };

    let items = vec!["apple", "banana", "cherry", "date", "elderberry"];

    // SAFE: Bounds check prevents out-of-range access
    if index >= items.len() { // vuln-code-snippet safe-line testcodeInputval005Safe
        return super::shared::BenchmarkResponse::bad_request("Index out of range");
    }

    super::shared::BenchmarkResponse::ok(&format!("Selected: {}", items[index]))
}
// vuln-code-snippet end testcodeInputval005Safe
