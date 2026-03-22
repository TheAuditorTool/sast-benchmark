//! CWE-190: Cast u64 to u32 via `as` keyword. Silently truncates high bits.

// vuln-code-snippet start testcodeIntoverflow003
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("value");
    let big_val: u64 = input.parse().unwrap_or(0);

    let small_val = big_val as u32; // vuln-code-snippet target-line testcodeIntoverflow003

    let msg = format!("Truncated {} -> {}", big_val, small_val);
    super::shared::BenchmarkResponse::ok(&msg)
}
// vuln-code-snippet end testcodeIntoverflow003
