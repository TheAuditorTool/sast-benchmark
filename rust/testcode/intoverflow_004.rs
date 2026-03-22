//! CWE-190: checked_mul returns None on overflow.

// vuln-code-snippet start testcodeIntoverflow004
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let a: i32 = req.param("a").parse().unwrap_or(0);
    let b: i32 = req.param("b").parse().unwrap_or(0);

    match a.checked_mul(b) { // vuln-code-snippet target-line testcodeIntoverflow004
        Some(result) => super::shared::BenchmarkResponse::ok(&format!("Product: {}", result)),
        None => super::shared::BenchmarkResponse::bad_request("Integer overflow"),
    }
}
// vuln-code-snippet end testcodeIntoverflow004
