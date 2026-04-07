//! CWE-502: User-supplied base64 decoded and then deserialized without validation.

// vuln-code-snippet start testcodeDeser034
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let encoded = req.param("data");
    let decoded = base64_decode(&encoded);
    let result = deserialize_object(&decoded); // vuln-code-snippet target-line testcodeDeser034
    super::shared::BenchmarkResponse::ok(&format!("len={}", result))
}

fn base64_decode(_s: &str) -> Vec<u8> { vec![0u8; 42] }
fn deserialize_object(_data: &[u8]) -> usize { 1 }
// vuln-code-snippet end testcodeDeser034
