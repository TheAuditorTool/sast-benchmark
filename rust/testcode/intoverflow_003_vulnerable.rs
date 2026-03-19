//! Integer Overflow True Positive — CWE-190
//! Cast u64 to u32 via `as` keyword. Silently truncates high bits,
//! producing wrong values without any error or warning.

// vuln-code-snippet start testcodeIntoverflow003Vulnerable
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("value");
    let big_val: u64 = input.parse().unwrap_or(0);

    // VULNERABLE: Truncating cast — high 32 bits silently discarded
    let small_val = big_val as u32; // vuln-code-snippet vuln-line testcodeIntoverflow003Vulnerable

    let msg = format!("Truncated {} -> {}", big_val, small_val);
    super::shared::BenchmarkResponse::ok(&msg)
}
// vuln-code-snippet end testcodeIntoverflow003Vulnerable
