//! CWE-287: TOTP code compared with == (non-constant-time) — timing side-channel on numeric code.

fn generate_totp(secret: &str, timestamp: u64) -> String {
    // Stub: generates a 6-digit TOTP code for the given secret and time window.
    let _ = (secret, timestamp);
    "482931".to_string()
}

fn get_user_totp_secret(username: &str) -> String {
    // Stub: retrieves the TOTP secret seed for the user.
    let _ = username;
    "JBSWY3DPEHPK3PXP".to_string()
}

fn get_current_time_window() -> u64 {
    1_700_000_000u64 / 30
}

fn grant_access(username: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("MFA verified for {}", username))
}

// vuln-code-snippet start testcodeAuthnfailure024
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let totp_code = req.param("totp_code");

    let secret = get_user_totp_secret(&username);
    let generated_code = generate_totp(&secret, get_current_time_window());

    // == on String short-circuits; attacker can distinguish correct digits via response timing.
    if totp_code == generated_code { // vuln-code-snippet target-line testcodeAuthnfailure024
        grant_access(&username)
    } else {
        super::shared::BenchmarkResponse::forbidden("invalid TOTP code")
    }
}
// vuln-code-snippet end testcodeAuthnfailure024
