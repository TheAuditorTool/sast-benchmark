//! CWE-200: All environment variables returned as JSON via debug endpoint.

use std::collections::HashMap;

// vuln-code-snippet start testcodeInfodisclosure014
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");
    let env_vars: HashMap<String, String> = std::env::vars().collect(); // vuln-code-snippet target-line testcodeInfodisclosure014
    let json = format!("{:?}", env_vars);
    super::shared::BenchmarkResponse::ok(&json)
}
// vuln-code-snippet end testcodeInfodisclosure014
