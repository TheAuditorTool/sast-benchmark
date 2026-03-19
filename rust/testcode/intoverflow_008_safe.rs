//! Integer Overflow True Negative — CWE-190
//! BigInt (simulated via i128) for arbitrary precision arithmetic.
//! No overflow possible within the extended range.

// vuln-code-snippet start testcodeIntoverflow008Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let a: i128 = req.param("a").parse().unwrap_or(0);
    let b: i128 = req.param("b").parse().unwrap_or(0);

    // SAFE: i128 (simulating BigInt) — no overflow for typical i32 inputs
    let result = a.checked_mul(b); // vuln-code-snippet safe-line testcodeIntoverflow008Safe

    match result {
        Some(v) => super::shared::BenchmarkResponse::ok(&format!("Product: {}", v)),
        None => super::shared::BenchmarkResponse::bad_request("Overflow even in i128"),
    }
}
// vuln-code-snippet end testcodeIntoverflow008Safe
