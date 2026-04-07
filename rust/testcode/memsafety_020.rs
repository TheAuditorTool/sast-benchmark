//! CWE-119: User-controlled length stored in struct and passed to slice::from_raw_parts.

// vuln-code-snippet start testcodeMemsafety020
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let spec = ReadSpec { len: req.param("len").parse().unwrap_or(0) };
    let data = [0u8; 128];
    let slice = unsafe { std::slice::from_raw_parts(data.as_ptr(), spec.len) }; // vuln-code-snippet target-line testcodeMemsafety020
    super::shared::BenchmarkResponse::ok(&format!("read={}", slice.len()))
}

struct ReadSpec { len: usize }
// vuln-code-snippet end testcodeMemsafety020
