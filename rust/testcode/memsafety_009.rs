//! CWE-119: copy_from_slice instead of ptr::copy. Automatic bounds checking.

// vuln-code-snippet start testcodeMemsafety009
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("data");
    let src = input.as_bytes();

    if src.len() > 256 {
        return super::shared::BenchmarkResponse::bad_request("Input too large");
    }

    let mut dst = vec![0u8; src.len()];

    dst.copy_from_slice(src); // vuln-code-snippet target-line testcodeMemsafety009

    super::shared::BenchmarkResponse::ok(&format!("Copied {} bytes", dst.len()))
}
// vuln-code-snippet end testcodeMemsafety009
