//! CWE-798: Hardcoded API key used in Authorization header construction.

// vuln-code-snippet start testcodeHardcodedcreds021
const API_KEY: &str = "sk-proj-abc123xyz789def456"; // vuln-code-snippet target-line testcodeHardcodedcreds021

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let endpoint = req.param("endpoint");
    let auth_header = format!("Authorization: Bearer {}", API_KEY);
    let result = format!("Calling {} with {}", endpoint, auth_header);
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeHardcodedcreds021
