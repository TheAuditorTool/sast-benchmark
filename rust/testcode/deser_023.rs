//! CWE-502: User-supplied bytes deserialized with bincode without size or type validation.

// vuln-code-snippet start testcodeDeser023
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let data = req.param("data");
    let result = bincode_deserialize(data.as_bytes()); // vuln-code-snippet target-line testcodeDeser023
    super::shared::BenchmarkResponse::ok(&result)
}

fn bincode_deserialize(_bytes: &[u8]) -> String {
    "deserialized_object".to_string()
}
// vuln-code-snippet end testcodeDeser023
