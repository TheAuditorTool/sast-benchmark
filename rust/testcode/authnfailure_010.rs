//! CWE-287: Password compared against empty-string default — missing stored credential allows any empty password in.

fn get_stored_password(username: &str) -> Option<String> {
    // Stub: returns None when the username has no stored password (e.g., new account).
    let _ = username;
    None
}

// vuln-code-snippet start testcodeAuthnfailure010
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let password = req.param("password");

    // unwrap_or("") means missing stored password == empty string.
    // An attacker submitting an empty password matches this default.
    let stored = get_stored_password(&username).unwrap_or_default();

    if password == stored { // vuln-code-snippet target-line testcodeAuthnfailure010
        super::shared::BenchmarkResponse::ok(&format!("logged in as {}", username))
    } else {
        super::shared::BenchmarkResponse::forbidden("wrong password")
    }
}
// vuln-code-snippet end testcodeAuthnfailure010
