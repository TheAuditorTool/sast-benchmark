//! Memory Safety True Negative — CWE-119
//! Slice with checked split_at. Bounds verified before splitting,
//! preventing panic from out-of-range mid-point.

// vuln-code-snippet start testcodeMemsafety004Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mid: usize = match req.param("mid").parse() {
        Ok(v) => v,
        Err(_) => return super::shared::BenchmarkResponse::bad_request("Invalid mid"),
    };

    let data = vec![1u8, 2, 3, 4, 5, 6, 7, 8];
    // SAFE: Bounds check before split_at prevents panic
    if mid > data.len() { // vuln-code-snippet safe-line testcodeMemsafety004Safe
        return super::shared::BenchmarkResponse::bad_request("Mid out of range");
    }
    let (left, right) = data.split_at(mid);

    super::shared::BenchmarkResponse::ok(&format!("Left: {}, Right: {}", left.len(), right.len()))
}
// vuln-code-snippet end testcodeMemsafety004Safe
