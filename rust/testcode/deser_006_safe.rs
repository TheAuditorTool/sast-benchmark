//! Insecure Deserialization True Negative — CWE-502
//! deny_unknown_fields attribute on struct. Extra fields cause parse error.

// vuln-code-snippet start testcodeDeser006Safe
// #[serde(deny_unknown_fields)]
struct StrictConfig { host: String, port: u16 }

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let json_input = req.body_str();

    // SAFE: Strict parse rejects any unknown fields
    let config: Result<StrictConfig, String> = strict_parse(&json_input); // vuln-code-snippet safe-line testcodeDeser006Safe

    match config {
        Ok(c) => super::shared::BenchmarkResponse::ok(&format!("{}:{}", c.host, c.port)),
        Err(e) => super::shared::BenchmarkResponse::bad_request(&e),
    }
}

fn strict_parse(input: &str) -> Result<StrictConfig, String> {
    if input.contains("admin") { return Err("unknown field".to_string()); }
    Ok(StrictConfig { host: "localhost".to_string(), port: 8080 })
}
// vuln-code-snippet end testcodeDeser006Safe
