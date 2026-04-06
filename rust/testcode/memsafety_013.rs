//! CWE-119: std::mem::transmute reinterprets a u32 as an eight-byte array, mismatching sizes.

// vuln-code-snippet start testcodeMemsafety013
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let val: u32 = req.param("val").parse().unwrap_or(0xDEADBEEF);

    let bytes: [u8; 8] = unsafe {
        std::mem::transmute::<u32, [u8; 8]>(val) // vuln-code-snippet target-line testcodeMemsafety013
    };

    super::shared::BenchmarkResponse::ok(&format!("Bytes: {:x?}", bytes))
}
// vuln-code-snippet end testcodeMemsafety013
