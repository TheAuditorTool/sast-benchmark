//! CWE-798: Bearer token default value assigned from string literal for HTTP client.

// vuln-code-snippet start testcodeHardcodedcreds009
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let endpoint = req.param("endpoint");
    let token = "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.dozjgNryP4J3jVmNHl0w5N_XgL0n3I9PlFUP0THsR8U"; // vuln-code-snippet target-line testcodeHardcodedcreds009
    // Simulates: reqwest::Client::new().get(&endpoint).header("Authorization", token)
    let result = format!("Calling {} with bearer token", endpoint);
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeHardcodedcreds009
