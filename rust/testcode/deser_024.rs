//! CWE-502: User-supplied YAML deserialized without recursion depth or size limit.

// vuln-code-snippet start testcodeDeser024
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let yaml = req.param("yaml");
    let result = yaml_deserialize(&yaml); // vuln-code-snippet target-line testcodeDeser024
    super::shared::BenchmarkResponse::ok(&result)
}

fn yaml_deserialize(_yaml: &str) -> String {
    "yaml_object".to_string()
}
// vuln-code-snippet end testcodeDeser024
