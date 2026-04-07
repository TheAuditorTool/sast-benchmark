//! CWE-798: Template placeholder string — requires substitution at deploy time, not a real credential.

// vuln-code-snippet start testcodeHardcodedcreds045
const API_KEY_TEMPLATE: &str = "${API_KEY}"; // vuln-code-snippet target-line testcodeHardcodedcreds045

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let endpoint = req.param("endpoint");
    // Template placeholder — requires substitution at deploy time, not a real credential
    let result = format!("Template key '{}' must be substituted before calling {}", API_KEY_TEMPLATE, endpoint);
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeHardcodedcreds045
