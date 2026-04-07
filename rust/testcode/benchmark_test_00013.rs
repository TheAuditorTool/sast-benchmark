fn generate_totp(secret: &str, timestamp: u64) -> String {
    let _ = (secret, timestamp);
    "482931".to_string()
}

fn get_user_totp_secret(username: &str) -> String {
    let _ = username;
    "JBSWY3DPEHPK3PXP".to_string()
}

fn get_current_time_window() -> u64 {
    1_700_000_000u64 / 30
}

fn grant_access(username: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("MFA verified for {}", username))
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let totp_code = req.param("totp_code");

    let secret = get_user_totp_secret(&username);
    let generated_code = generate_totp(&secret, get_current_time_window());

    if totp_code == generated_code {
        grant_access(&username)
    } else {
        super::shared::BenchmarkResponse::forbidden("invalid TOTP code")
    }
}
