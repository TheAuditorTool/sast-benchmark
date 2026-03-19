//! Memory Safety True Negative — CWE-119
//! copy_from_slice instead of ptr::copy. Safe slice copy with automatic
//! bounds checking — panics on mismatch rather than causing UB.

// vuln-code-snippet start testcodeMemsafety009Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("data");
    let src = input.as_bytes();

    if src.len() > 256 {
        return super::shared::BenchmarkResponse::bad_request("Input too large");
    }

    let mut dst = vec![0u8; src.len()];
    // SAFE: copy_from_slice checks lengths match — no OOB write
    dst.copy_from_slice(src); // vuln-code-snippet safe-line testcodeMemsafety009Safe

    super::shared::BenchmarkResponse::ok(&format!("Copied {} bytes", dst.len()))
}
// vuln-code-snippet end testcodeMemsafety009Safe
