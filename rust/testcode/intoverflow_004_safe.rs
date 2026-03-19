//! Integer Overflow True Negative — CWE-190
//! checked_mul returns None on overflow instead of wrapping.
//! Caller handles the None case explicitly.

// vuln-code-snippet start testcodeIntoverflow004Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let a: i32 = req.param("a").parse().unwrap_or(0);
    let b: i32 = req.param("b").parse().unwrap_or(0);

    // SAFE: checked_mul returns None on overflow
    match a.checked_mul(b) { // vuln-code-snippet safe-line testcodeIntoverflow004Safe
        Some(result) => super::shared::BenchmarkResponse::ok(&format!("Product: {}", result)),
        None => super::shared::BenchmarkResponse::bad_request("Integer overflow"),
    }
}
// vuln-code-snippet end testcodeIntoverflow004Safe
