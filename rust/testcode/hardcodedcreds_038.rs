//! CWE-798: JWT secret read from environment variable; returns error response if unset.

// vuln-code-snippet start testcodeHardcodedcreds038
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user = req.param("user");
    let jwt_secret = match std::env::var("JWT_SECRET") { // vuln-code-snippet target-line testcodeHardcodedcreds038
        Ok(s) => s,
        Err(_) => return super::shared::BenchmarkResponse::error("JWT_SECRET not configured"),
    };
    let token = format!("sign({}, secret_len={})", user, jwt_secret.len());
    super::shared::BenchmarkResponse::ok(&token)
}
// vuln-code-snippet end testcodeHardcodedcreds038
