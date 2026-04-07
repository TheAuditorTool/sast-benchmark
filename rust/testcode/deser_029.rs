//! CWE-502: User-supplied bytes deserialized with MessagePack without validation.

// vuln-code-snippet start testcodeDeser029
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let data = req.param("data");
    let result = msgpack_deserialize(data.as_bytes()); // vuln-code-snippet target-line testcodeDeser029
    super::shared::BenchmarkResponse::ok(&result)
}

fn msgpack_deserialize(_bytes: &[u8]) -> String {
    "msgpack_object".to_string()
}
// vuln-code-snippet end testcodeDeser029
