//! CWE-190: Buffer index calculated by adding offset and length without overflow check.

// vuln-code-snippet start testcodeIntoverflow024
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let offset: usize = req.param("offset").parse().unwrap_or(0);
    let length: usize = req.param("length").parse().unwrap_or(0);
    let end_index = offset + length; // vuln-code-snippet target-line testcodeIntoverflow024
    super::shared::BenchmarkResponse::ok(&format!("end={}", end_index))
}
// vuln-code-snippet end testcodeIntoverflow024
