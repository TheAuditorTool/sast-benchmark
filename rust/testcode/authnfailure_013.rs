//! CWE-287: X-Debug-Auth header value trusted to bypass authentication and grant admin access.

fn admin_access(username: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("admin panel for {}", username))
}

fn verify_admin_credentials(username: &str, password: &str) -> bool {
    let _ = (username, password);
    false
}

// vuln-code-snippet start testcodeAuthnfailure013
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let password = req.param("password");

    // Attacker-controlled header used to skip credential verification.
    if req.header("X-Debug-Auth") == "true" {
        return admin_access(&username); // vuln-code-snippet target-line testcodeAuthnfailure013
    }

    if verify_admin_credentials(&username, &password) {
        admin_access(&username)
    } else {
        super::shared::BenchmarkResponse::forbidden("forbidden")
    }
}
// vuln-code-snippet end testcodeAuthnfailure013
