//! Input Validation True Negative — CWE-20
//! Allocation size capped at MAX_SIZE constant.

const MAX_SIZE: usize = 1_048_576; // 1 MB

// vuln-code-snippet start testcodeInputval006Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let size_str = req.param("size");
    let size: usize = match size_str.parse() {
        Ok(v) => v,
        Err(_) => return super::shared::BenchmarkResponse::bad_request("Invalid size"),
    };

    // SAFE: Allocation capped to prevent OOM from user-controlled size
    let capped = std::cmp::min(size, MAX_SIZE); // vuln-code-snippet safe-line testcodeInputval006Safe
    let buffer: Vec<u8> = vec![0u8; capped];

    super::shared::BenchmarkResponse::ok(&format!("Allocated {} bytes (requested {})", buffer.len(), size))
}
// vuln-code-snippet end testcodeInputval006Safe
