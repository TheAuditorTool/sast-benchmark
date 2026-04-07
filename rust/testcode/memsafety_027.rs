//! CWE-119: Safe bounds-checked get() returns Option; no unsafe memory access possible.

// vuln-code-snippet start testcodeMemsafety027
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let idx: usize = req.param("index").parse().unwrap_or(0);
    let data = [1u8, 2, 3, 4, 5, 6, 7, 8];
    match data.get(idx) {
        Some(val) => super::shared::BenchmarkResponse::ok(&format!("val={}", val)), // vuln-code-snippet target-line testcodeMemsafety027
        None => super::shared::BenchmarkResponse::bad_request("Out of bounds"),
    }
}
// vuln-code-snippet end testcodeMemsafety027
