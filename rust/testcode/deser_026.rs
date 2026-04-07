//! CWE-502: User-provided CBOR bytes deserialized without validation.

// vuln-code-snippet start testcodeDeser026
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let data = req.param("data");
    let result = cbor_deserialize(data.as_bytes()); // vuln-code-snippet target-line testcodeDeser026
    super::shared::BenchmarkResponse::ok(&result)
}

fn cbor_deserialize(_bytes: &[u8]) -> String {
    "cbor_object".to_string()
}
// vuln-code-snippet end testcodeDeser026
