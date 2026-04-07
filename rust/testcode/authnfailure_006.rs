//! CWE-287: Password verification skipped — authenticate_user called regardless of password value.

fn authenticate_user(username: &str) -> super::shared::BenchmarkResponse {
    // Stub: creates an authenticated session for the given username unconditionally.
    super::shared::BenchmarkResponse::ok(&format!("logged in as {}", username))
}

// vuln-code-snippet start testcodeAuthnfailure006
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let _password = req.param("password");

    // Password read but never checked; any value (including empty) grants access.

    let response = authenticate_user(&username); // vuln-code-snippet target-line testcodeAuthnfailure006

    response
}
// vuln-code-snippet end testcodeAuthnfailure006
