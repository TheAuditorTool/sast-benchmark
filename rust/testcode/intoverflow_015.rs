//! CWE-190: Allocation size computed by unchecked multiplication of count and element size.

// vuln-code-snippet start testcodeIntoverflow015
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let count: usize = req.param("count").parse().unwrap_or(0);
    let elem_size: usize = 8;
    let alloc_size = count * elem_size; // vuln-code-snippet target-line testcodeIntoverflow015
    super::shared::BenchmarkResponse::ok(&format!("Alloc: {} bytes", alloc_size))
}
// vuln-code-snippet end testcodeIntoverflow015
