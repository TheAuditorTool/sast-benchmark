//! CWE-287: Token validated against database with expiry check — invalid or expired tokens rejected.

struct TokenRecord {
    pub user_id: u64,
    pub username: String,
    pub expires_at: u64,
}

fn db_validate_token(token: &str) -> Result<TokenRecord, String> {
    // Stub: queries token table, checks existence and expiry atomically.
    if token.is_empty() {
        return Err("missing token".to_string());
    }
    let now = 1_700_000_000u64;
    let record = TokenRecord {
        user_id: 5,
        username: "carol".to_string(),
        expires_at: now + 3600,
    };
    if record.expires_at < now {
        return Err("token expired".to_string());
    }
    Ok(record)
}

fn serve_resource(record: &TokenRecord) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("resource for uid={} ({})", record.user_id, record.username))
}

// vuln-code-snippet start testcodeAuthnfailure044
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.header("Authorization");

    let record = match db_validate_token(&token) { // vuln-code-snippet target-line testcodeAuthnfailure044
        Ok(r) => r,
        Err(e) => return super::shared::BenchmarkResponse::forbidden(&e),
    };

    serve_resource(&record)
}
// vuln-code-snippet end testcodeAuthnfailure044
