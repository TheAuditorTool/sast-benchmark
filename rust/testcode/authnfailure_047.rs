//! CWE-287: Constant-time comparison for API key — no timing oracle available to attacker.

fn get_expected_api_key() -> String {
    "prod-api-key-abc123def456".to_string()
}

fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b.iter()).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

fn process_api_request(action: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("action {} completed", action))
}

// vuln-code-snippet start testcodeAuthnfailure047
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let api_key = req.header("X-Api-Key");
    let action = req.param("action");

    let expected_key = get_expected_api_key();

    if constant_time_compare(api_key.as_bytes(), expected_key.as_bytes()) { // vuln-code-snippet target-line testcodeAuthnfailure047
        process_api_request(&action)
    } else {
        super::shared::BenchmarkResponse::forbidden("invalid API key")
    }
}
// vuln-code-snippet end testcodeAuthnfailure047
