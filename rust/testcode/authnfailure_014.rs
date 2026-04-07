//! CWE-287: URL parameter "bypass=1" skips authentication entirely.

fn authenticated(username: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("authenticated: {}", username))
}

fn verify_session(session_token: &str) -> bool {
    let _ = session_token;
    false
}

// vuln-code-snippet start testcodeAuthnfailure014
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let session = req.cookie("session");

    // Query parameter controlled by attacker bypasses session verification.
    if req.param("bypass") == "1" {
        return authenticated(&username); // vuln-code-snippet target-line testcodeAuthnfailure014
    }

    if verify_session(&session) {
        authenticated(&username)
    } else {
        super::shared::BenchmarkResponse::forbidden("no valid session")
    }
}
// vuln-code-snippet end testcodeAuthnfailure014
