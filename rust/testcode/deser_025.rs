//! CWE-502: Deserialization attempted without checking data size; oversized payload can exhaust memory.

// vuln-code-snippet start testcodeDeser025
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.param("body");
    let obj = deserialize_object(body.as_bytes()); // vuln-code-snippet target-line testcodeDeser025
    super::shared::BenchmarkResponse::ok(&format!("deserialized len={}", obj))
}

fn deserialize_object(_data: &[u8]) -> usize { 42 }
// vuln-code-snippet end testcodeDeser025
