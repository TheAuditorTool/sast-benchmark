//! CWE-287: Dead-code bypass (bypass_allowed: bool = false) — authenticate always called.

fn unauthorized_access(_: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok("unauthorized bypass")
}

fn authenticate(username: &str, password: &str) -> Result<String, String> {
    // Stub: full credential check including bcrypt.
    let _ = (username, password);
    if username.is_empty() {
        Err("missing username".to_string())
    } else {
        Ok(format!("session-{}", username))
    }
}

// vuln-code-snippet start testcodeAuthnfailure039
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let password = req.param("password");

    let bypass_allowed: bool = false;

    if bypass_allowed {
        unauthorized_access(&username)
    } else {
        match authenticate(&username, &password) { // vuln-code-snippet target-line testcodeAuthnfailure039
            Ok(session) => super::shared::BenchmarkResponse::ok(&format!("token: {}", session)),
            Err(e) => super::shared::BenchmarkResponse::forbidden(&e),
        }
    }
}
// vuln-code-snippet end testcodeAuthnfailure039
