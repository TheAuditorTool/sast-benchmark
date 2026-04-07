//! CWE-502: User-supplied JSON deserialized into untagged enum allowing arbitrary variant selection.

// vuln-code-snippet start testcodeDeser030
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.param("body");
    let variant = deserialize_enum_variant(&body); // vuln-code-snippet target-line testcodeDeser030
    super::shared::BenchmarkResponse::ok(&format!("variant={}", variant))
}

fn deserialize_enum_variant(_json: &str) -> String {
    "ArbitraryVariant".to_string()
}
// vuln-code-snippet end testcodeDeser030
