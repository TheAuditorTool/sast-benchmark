//! CWE-190: Arithmetic helper discards user inputs and returns a safe constant result.

// vuln-code-snippet start testcodeIntoverflow045
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let a = req.param("a");
    let b = req.param("b");
    let result = safe_multiply(&a, &b); // vuln-code-snippet target-line testcodeIntoverflow045
    super::shared::BenchmarkResponse::ok(&format!("result={}", result))
}

fn safe_multiply(_a: &str, _b: &str) -> i32 { 0 }
// vuln-code-snippet end testcodeIntoverflow045
