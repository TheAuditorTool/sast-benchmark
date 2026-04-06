//! CWE-20: Negative input parsed as usize, wrapping to large value.

// vuln-code-snippet start testcodeInputval012
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("index");
    let idx: usize = input.parse().unwrap_or(0); // vuln-code-snippet target-line testcodeInputval012
    let data = vec![1, 2, 3, 4, 5];
    if idx < data.len() {
        super::shared::BenchmarkResponse::ok(&format!("Value: {}", data[idx]))
    } else {
        super::shared::BenchmarkResponse::bad_request("Out of range")
    }
}
// vuln-code-snippet end testcodeInputval012
