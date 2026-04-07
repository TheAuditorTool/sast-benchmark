//! CWE-798: Hardcoded OAuth client secret stored in a const used in OAuth flow.

// vuln-code-snippet start testcodeHardcodedcreds029
const OAUTH_CLIENT_SECRET: &str = "oauth-client-secret-prod-2024xyz"; // vuln-code-snippet target-line testcodeHardcodedcreds029

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let code = req.param("code");
    let result = format!(
        "POST /oauth/token client_secret={} code={}",
        OAUTH_CLIENT_SECRET, code
    );
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeHardcodedcreds029
