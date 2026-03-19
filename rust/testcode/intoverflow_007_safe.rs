//! Integer Overflow True Negative — CWE-190
//! Range validated before arithmetic. Inputs clamped to known-safe
//! bounds so multiplication cannot overflow.

// vuln-code-snippet start testcodeIntoverflow007Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let a: i32 = req.param("a").parse().unwrap_or(0);
    let b: i32 = req.param("b").parse().unwrap_or(0);

    // SAFE: Range check ensures multiplication cannot overflow
    if a.abs() > 46340 || b.abs() > 46340 { // vuln-code-snippet safe-line testcodeIntoverflow007Safe
        return super::shared::BenchmarkResponse::bad_request("Input out of safe range");
    }

    let result = a * b;
    super::shared::BenchmarkResponse::ok(&format!("Product: {}", result))
}
// vuln-code-snippet end testcodeIntoverflow007Safe
