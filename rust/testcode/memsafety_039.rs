//! CWE-119: Buffer access uses safe Rust range indexing with built-in bounds checking.

// vuln-code-snippet start testcodeMemsafety039
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let idx: usize = req.param("index").parse().unwrap_or(0);
    let data = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    match data.get(idx..idx.saturating_add(4)) {
        Some(chunk) => super::shared::BenchmarkResponse::ok(&format!("chunk_len={}", chunk.len())), // vuln-code-snippet target-line testcodeMemsafety039
        None => super::shared::BenchmarkResponse::bad_request("Out of bounds"),
    }
}
// vuln-code-snippet end testcodeMemsafety039
