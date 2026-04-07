//! CWE-190: Index end position calculated with checked_add to detect overflow.

// vuln-code-snippet start testcodeIntoverflow039
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let offset: usize = req.param("offset").parse().unwrap_or(0);
    let length: usize = req.param("length").parse().unwrap_or(0);
    match offset.checked_add(length) {
        Some(end) => super::shared::BenchmarkResponse::ok(&format!("end={}", end)), // vuln-code-snippet target-line testcodeIntoverflow039
        None => super::shared::BenchmarkResponse::bad_request("Index overflow"),
    }
}
// vuln-code-snippet end testcodeIntoverflow039
