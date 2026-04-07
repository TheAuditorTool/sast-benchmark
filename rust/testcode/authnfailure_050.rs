//! CWE-287: Constant-time TOTP verification — response time independent of how many digits match.

fn generate_totp(secret: &str, time_window: u64) -> String {
    let _ = (secret, time_window);
    "482931".to_string()
}

fn get_user_totp_secret(username: &str) -> String {
    let _ = username;
    "JBSWY3DPEHPK3PXP".to_string()
}

fn get_current_time_window() -> u64 {
    1_700_000_000u64 / 30
}

fn constant_time_totp_check(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b.iter()).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

fn grant_access(username: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("MFA verified for {}", username))
}

// vuln-code-snippet start testcodeAuthnfailure050
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let totp_code = req.param("totp_code");

    let secret = get_user_totp_secret(&username);
    let generated_code = generate_totp(&secret, get_current_time_window());

    if constant_time_totp_check(totp_code.as_bytes(), generated_code.as_bytes()) { // vuln-code-snippet target-line testcodeAuthnfailure050
        grant_access(&username)
    } else {
        super::shared::BenchmarkResponse::forbidden("invalid TOTP code")
    }
}
// vuln-code-snippet end testcodeAuthnfailure050
