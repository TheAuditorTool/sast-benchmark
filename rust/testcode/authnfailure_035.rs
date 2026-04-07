//! CWE-287: Multi-factor authentication — password AND TOTP both verified before access granted.

fn bcrypt_verify(password: &str, hash: &str) -> Result<bool, String> {
    let _ = (password, hash);
    Ok(password == "correct")
}

fn get_stored_hash(username: &str) -> Result<String, String> {
    let _ = username;
    Ok("$2b$12$fakehash".to_string())
}

fn get_totp_secret(username: &str) -> Result<String, String> {
    let _ = username;
    Ok("JBSWY3DPEHPK3PXP".to_string())
}

fn verify_totp(secret: &str, code: &str) -> bool {
    // Stub: constant-time TOTP verification.
    let _ = (secret, code);
    code == "482931"
}

fn grant_access(username: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("MFA passed, access granted: {}", username))
}

// vuln-code-snippet start testcodeAuthnfailure035
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let password = req.param("password");
    let totp_code = req.param("totp_code");

    let hash = match get_stored_hash(&username) {
        Ok(h) => h,
        Err(_) => return super::shared::BenchmarkResponse::forbidden("user not found"),
    };

    let pw_ok = match bcrypt_verify(&password, &hash) {
        Ok(v) => v,
        Err(_) => return super::shared::BenchmarkResponse::error("internal error"),
    };

    if !pw_ok {
        return super::shared::BenchmarkResponse::forbidden("wrong password");
    }

    let totp_secret = match get_totp_secret(&username) {
        Ok(s) => s,
        Err(_) => return super::shared::BenchmarkResponse::error("totp config error"),
    };

    if !verify_totp(&totp_secret, &totp_code) {
        return super::shared::BenchmarkResponse::forbidden("invalid TOTP code");
    }

    grant_access(&username) // vuln-code-snippet target-line testcodeAuthnfailure035
}
// vuln-code-snippet end testcodeAuthnfailure035
