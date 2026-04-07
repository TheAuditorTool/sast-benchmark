//! CWE-287: Session expiry explicitly checked — expired sessions rejected before granting access.

struct Session {
    pub username: String,
    pub expires_at: u64,
}

fn db_lookup_session(token: &str) -> Result<Session, String> {
    // Stub: retrieves session record from store.
    if token.is_empty() {
        return Err("missing session".to_string());
    }
    Ok(Session { username: "bob".to_string(), expires_at: 9_999_999_999u64 })
}

fn now() -> u64 {
    1_700_000_000u64
}

fn serve_content(username: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("content for {}", username))
}

// vuln-code-snippet start testcodeAuthnfailure034
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.cookie("session");

    let session = match db_lookup_session(&token) {
        Ok(s) => s,
        Err(e) => return super::shared::BenchmarkResponse::forbidden(&e),
    };

    if session.expires_at < now() { // vuln-code-snippet target-line testcodeAuthnfailure034
        return super::shared::BenchmarkResponse::forbidden("session expired");
    }

    serve_content(&session.username)
}
// vuln-code-snippet end testcodeAuthnfailure034
