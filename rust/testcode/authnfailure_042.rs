//! CWE-287: Role fetched from database after successful auth — never trusted from request parameters.

fn verify_session_and_get_user_id(token: &str) -> Result<u64, String> {
    // Stub: validates session token and returns the authenticated user's database ID.
    if token.is_empty() {
        return Err("invalid session".to_string());
    }
    Ok(42u64)
}

fn db_get_role(user_id: u64) -> Result<String, String> {
    // Stub: fetches the user's role from the authoritative user record in the database.
    if user_id == 0 {
        return Err("user not found".to_string());
    }
    Ok("user".to_string())
}

fn render_panel(user_id: u64, role: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("panel uid={} role={}", user_id, role))
}

// vuln-code-snippet start testcodeAuthnfailure042
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.cookie("session");

    let verified_user_id = match verify_session_and_get_user_id(&token) {
        Ok(id) => id,
        Err(e) => return super::shared::BenchmarkResponse::forbidden(&e),
    };

    let role = match db_get_role(verified_user_id) { // vuln-code-snippet target-line testcodeAuthnfailure042
        Ok(r) => r,
        Err(e) => return super::shared::BenchmarkResponse::error(&e),
    };

    render_panel(verified_user_id, &role)
}
// vuln-code-snippet end testcodeAuthnfailure042
