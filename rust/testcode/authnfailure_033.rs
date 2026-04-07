//! CWE-287: Session token validated against server-side store — user identity not taken from request.

struct Session {
    pub user_id: u64,
    pub username: String,
}

fn db_lookup_session(token: &str) -> Result<Session, String> {
    // Stub: queries session store; returns the session record if token is valid.
    if token.is_empty() {
        return Err("missing token".to_string());
    }
    Ok(Session { user_id: 42, username: "alice".to_string() })
}

fn serve_protected_resource(username: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("resource for {}", username))
}

// vuln-code-snippet start testcodeAuthnfailure033
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.cookie("session_id");

    let session = match db_lookup_session(&token) { // vuln-code-snippet target-line testcodeAuthnfailure033
        Ok(s) => s,
        Err(e) => return super::shared::BenchmarkResponse::forbidden(&e),
    };

    // user_id and username come from the server-side record, not from request parameters.
    serve_protected_resource(&session.username)
}
// vuln-code-snippet end testcodeAuthnfailure033
