//! CWE-798: Hardcoded bearer token literal embedded directly in a format! call.

// vuln-code-snippet start testcodeHardcodedcreds028
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let endpoint = req.param("endpoint");
    let auth_header = format!("Bearer {}", "hc_live_token_abc123def456"); // vuln-code-snippet target-line testcodeHardcodedcreds028
    let result = format!("GET {} with {}", endpoint, auth_header);
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeHardcodedcreds028
