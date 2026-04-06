//! CWE-190: Generic checked multiplication via CheckedMul trait.

// vuln-code-snippet start testcodeIntoverflow020
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let a: u64 = req.param("a").parse().unwrap_or(0);
    let b: u64 = req.param("b").parse().unwrap_or(0);
    match a.checked_mul(b) { // vuln-code-snippet target-line testcodeIntoverflow020
        Some(v) => super::shared::BenchmarkResponse::ok(&format!("Product: {}", v)),
        None => super::shared::BenchmarkResponse::bad_request("Multiplication overflow"),
    }
}
// vuln-code-snippet end testcodeIntoverflow020
