//! CWE-190: Range validated before arithmetic.

// vuln-code-snippet start testcodeIntoverflow007
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let a: i32 = req.param("a").parse().unwrap_or(0);
    let b: i32 = req.param("b").parse().unwrap_or(0);

    if a.abs() > 46340 || b.abs() > 46340 { // vuln-code-snippet target-line testcodeIntoverflow007
        return super::shared::BenchmarkResponse::bad_request("Input out of safe range");
    }

    let result = a * b;
    super::shared::BenchmarkResponse::ok(&format!("Product: {}", result))
}
// vuln-code-snippet end testcodeIntoverflow007
