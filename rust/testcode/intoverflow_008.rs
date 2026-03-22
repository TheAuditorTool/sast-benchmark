//! CWE-190: i128 (simulating BigInt) for extended precision arithmetic.

// vuln-code-snippet start testcodeIntoverflow008
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let a: i128 = req.param("a").parse().unwrap_or(0);
    let b: i128 = req.param("b").parse().unwrap_or(0);

    let result = a.checked_mul(b); // vuln-code-snippet target-line testcodeIntoverflow008

    match result {
        Some(v) => super::shared::BenchmarkResponse::ok(&format!("Product: {}", v)),
        None => super::shared::BenchmarkResponse::bad_request("Overflow even in i128"),
    }
}
// vuln-code-snippet end testcodeIntoverflow008
