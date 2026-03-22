//! CWE-119: from_raw_parts with user-controlled length.

// vuln-code-snippet start testcodeMemsafety001
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let len: usize = match req.param("len").parse() {
        Ok(v) => v,
        Err(_) => return super::shared::BenchmarkResponse::bad_request("Invalid len"),
    };

    let buf: Vec<u8> = vec![0u8; 64];
    let slice = unsafe { std::slice::from_raw_parts(buf.as_ptr(), len) }; // vuln-code-snippet target-line testcodeMemsafety001

    super::shared::BenchmarkResponse::ok(&format!("Read {} bytes", slice.len()))
}
// vuln-code-snippet end testcodeMemsafety001
