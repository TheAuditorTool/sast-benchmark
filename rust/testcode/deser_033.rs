//! CWE-502: Arbitrary object deserialized from user data allowing type confusion attacks.

// vuln-code-snippet start testcodeDeser033
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let data = req.param("data");
    let obj = arbitrary_deserialize(data.as_bytes()); // vuln-code-snippet target-line testcodeDeser033
    super::shared::BenchmarkResponse::ok(&format!("type={}", obj))
}

fn arbitrary_deserialize(_bytes: &[u8]) -> String {
    "ArbitraryObject".to_string()
}
// vuln-code-snippet end testcodeDeser033
