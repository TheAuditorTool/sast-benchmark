//! CWE-502: User-supplied payload stored in struct and deserialized without size or type check.

// vuln-code-snippet start testcodeDeser027
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let payload = DeserPayload { data: req.param("data") };
    let result = deserialize_payload(&payload); // vuln-code-snippet target-line testcodeDeser027
    super::shared::BenchmarkResponse::ok(&result)
}

struct DeserPayload { data: String }

fn deserialize_payload(p: &DeserPayload) -> String {
    format!("deserialized len={}", p.data.len())
}
// vuln-code-snippet end testcodeDeser027
