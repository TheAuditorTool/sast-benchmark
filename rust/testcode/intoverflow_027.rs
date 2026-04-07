//! CWE-190: Buffer size computed by multiplying user values stored in struct without overflow check.

// vuln-code-snippet start testcodeIntoverflow027
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let rows: usize = req.param("rows").parse().unwrap_or(0);
    let cols: usize = req.param("cols").parse().unwrap_or(0);
    let spec = BufSpec { size: rows * cols }; // vuln-code-snippet target-line testcodeIntoverflow027
    super::shared::BenchmarkResponse::ok(&format!("size={}", spec.size))
}

struct BufSpec { size: usize }
// vuln-code-snippet end testcodeIntoverflow027
