//! CWE-287: OR logic in credential check — correct username OR correct password is sufficient to authenticate.

fn get_stored_credentials() -> (String, String) {
    ("alice".to_string(), "s3cr3t".to_string())
}

fn return_authenticated(username: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("welcome {}", username))
}

// vuln-code-snippet start testcodeAuthnfailure009
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let password = req.param("password");

    let (stored_user, stored_pass) = get_stored_credentials();

    // OR instead of AND: knowing either the username or the password is enough.
    if username == stored_user || password == stored_pass { // vuln-code-snippet target-line testcodeAuthnfailure009
        return return_authenticated(&username);
    }

    super::shared::BenchmarkResponse::forbidden("invalid credentials")
}
// vuln-code-snippet end testcodeAuthnfailure009
