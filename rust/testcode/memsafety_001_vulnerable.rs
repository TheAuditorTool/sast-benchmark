//! Memory Safety True Positive — CWE-119
//! from_raw_parts with user-controlled length. Creates a slice from a raw
//! pointer using an attacker-supplied length, enabling out-of-bounds read.

// vuln-code-snippet start testcodeMemsafety001Vulnerable
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let len: usize = match req.param("len").parse() {
        Ok(v) => v,
        Err(_) => return super::shared::BenchmarkResponse::bad_request("Invalid len"),
    };

    let buf: Vec<u8> = vec![0u8; 64];
    // VULNERABLE: User-controlled length passed to from_raw_parts — OOB read
    let slice = unsafe { std::slice::from_raw_parts(buf.as_ptr(), len) }; // vuln-code-snippet vuln-line testcodeMemsafety001Vulnerable

    super::shared::BenchmarkResponse::ok(&format!("Read {} bytes", slice.len()))
}
// vuln-code-snippet end testcodeMemsafety001Vulnerable
