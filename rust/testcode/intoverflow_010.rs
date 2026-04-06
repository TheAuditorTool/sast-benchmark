//! CWE-190: Truncating cast from u64 to u32 via as operator on user input.

// vuln-code-snippet start testcodeIntoverflow010
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let big_val: u64 = req.param("value").parse().unwrap_or(0);
    let small = big_val as u32; // vuln-code-snippet target-line testcodeIntoverflow010
    super::shared::BenchmarkResponse::ok(&format!("Truncated: {}", small))
}
// vuln-code-snippet end testcodeIntoverflow010
