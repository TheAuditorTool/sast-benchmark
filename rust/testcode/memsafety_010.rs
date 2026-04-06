//! CWE-119: Vec::set_len used to extend length beyond allocated capacity.

// vuln-code-snippet start testcodeMemsafety010
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _input = req.param("data");

    let mut vec: Vec<u8> = Vec::with_capacity(64);

    unsafe {
        vec.set_len(vec.capacity() + 100); // vuln-code-snippet target-line testcodeMemsafety010
    }

    super::shared::BenchmarkResponse::ok(&format!("Vec length: {}", vec.len()))
}
// vuln-code-snippet end testcodeMemsafety010
