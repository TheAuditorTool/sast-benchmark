//! CWE-287: API key compared with == (non-constant-time) — susceptible to timing side-channel.

fn get_expected_api_key() -> String {
    // Stub: returns the configured expected API key.
    "prod-api-key-abc123def456".to_string()
}

fn process_api_request(action: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("action {} completed", action))
}

// vuln-code-snippet start testcodeAuthnfailure022
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let api_key = req.header("X-Api-Key");
    let action = req.param("action");

    let expected_key = get_expected_api_key();

    // Non-constant-time equality check allows timing oracle to recover key byte-by-byte.
    if api_key == expected_key { // vuln-code-snippet target-line testcodeAuthnfailure022
        process_api_request(&action)
    } else {
        super::shared::BenchmarkResponse::forbidden("invalid API key")
    }
}
// vuln-code-snippet end testcodeAuthnfailure022
