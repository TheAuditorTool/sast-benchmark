//! CWE-798: OAuth client secret assigned from string literal.

// vuln-code-snippet start testcodeHardcodedcreds007
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let auth_code = req.param("code");
    let client_secret = "dGhpcyBpcyBhIHNlY3JldA=="; // vuln-code-snippet target-line testcodeHardcodedcreds007
    // Simulates: oauth2::ClientSecret::new(client_secret.to_string())
    let result = format!("Exchanging code {} with secret {}...", auth_code, &client_secret[..8]);
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeHardcodedcreds007
