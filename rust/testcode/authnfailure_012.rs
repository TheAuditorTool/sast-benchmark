//! CWE-287: Hardcoded master password grants super-admin access, bypassing normal auth.

fn verify_password(username: &str, password: &str) -> bool {
    let _ = (username, password);
    false
}

fn super_admin(username: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("super-admin session for {}", username))
}

fn normal_login(username: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("user session for {}", username))
}

// vuln-code-snippet start testcodeAuthnfailure012
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let password = req.param("password");

    // Hardcoded master password bypasses all per-user credential verification.
    if password == "master-override-key" { // vuln-code-snippet target-line testcodeAuthnfailure012
        return super_admin(&username);
    }

    if verify_password(&username, &password) {
        normal_login(&username)
    } else {
        super::shared::BenchmarkResponse::forbidden("invalid credentials")
    }
}
// vuln-code-snippet end testcodeAuthnfailure012
