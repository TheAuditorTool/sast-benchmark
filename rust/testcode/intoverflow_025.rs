//! CWE-190: Large u64 value cast to u32 truncates upper 32 bits silently.

// vuln-code-snippet start testcodeIntoverflow025
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let large: u64 = req.param("value").parse().unwrap_or(0);
    let truncated = large as u32; // vuln-code-snippet target-line testcodeIntoverflow025
    super::shared::BenchmarkResponse::ok(&format!("truncated={}", truncated))
}
// vuln-code-snippet end testcodeIntoverflow025
