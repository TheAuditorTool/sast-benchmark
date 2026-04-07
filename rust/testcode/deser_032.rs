//! CWE-502: Content-Type verified as JSON but payload deserialized into untyped Value without schema.

// vuln-code-snippet start testcodeDeser032
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let ct = req.header("Content-Type");
    if !ct.contains("application/json") {
        return super::shared::BenchmarkResponse::bad_request("Not JSON");
    }
    let body = req.param("body");
    let value = parse_json_value(&body); // vuln-code-snippet target-line testcodeDeser032
    super::shared::BenchmarkResponse::ok(&value)
}

fn parse_json_value(_json: &str) -> String {
    "serde_json::Value::Object(...)".to_string()
}
// vuln-code-snippet end testcodeDeser032
