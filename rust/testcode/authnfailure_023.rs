//! CWE-287: Password reset token compared with == (non-constant-time) — timing side-channel leaks token.

fn get_valid_reset_token(email: &str) -> String {
    // Stub: retrieves the stored password reset token for the given email.
    let _ = email;
    "reset-token-xyz789abc".to_string()
}

fn complete_password_reset(email: &str, new_password: &str) -> super::shared::BenchmarkResponse {
    let _ = new_password;
    super::shared::BenchmarkResponse::ok(&format!("password reset for {}", email))
}

// vuln-code-snippet start testcodeAuthnfailure023
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let email = req.param("email");
    let reset_token = req.param("token");
    let new_password = req.param("new_password");

    let valid_token = get_valid_reset_token(&email);

    // Short-circuit string comparison leaks how many characters of the reset token are correct.
    if reset_token == valid_token { // vuln-code-snippet target-line testcodeAuthnfailure023
        complete_password_reset(&email, &new_password)
    } else {
        super::shared::BenchmarkResponse::forbidden("invalid or expired reset token")
    }
}
// vuln-code-snippet end testcodeAuthnfailure023
