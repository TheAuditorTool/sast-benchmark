//! CWE-287: Session looked up in server-side store — user identity never taken from request parameters.

struct UserRecord {
    pub username: String,
    pub role: String,
}

fn session_store_lookup(session_id: &str) -> Result<UserRecord, String> {
    // Stub: queries the session store and returns the associated user record.
    if session_id.is_empty() {
        return Err("missing session".to_string());
    }
    Ok(UserRecord { username: "alice".to_string(), role: "user".to_string() })
}

fn serve_dashboard(user: &UserRecord) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("dashboard for {} ({})", user.username, user.role))
}

// vuln-code-snippet start testcodeAuthnfailure041
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let session_id = req.cookie("session_id");

    let user = match session_store_lookup(&session_id) { // vuln-code-snippet target-line testcodeAuthnfailure041
        Ok(u) => u,
        Err(e) => return super::shared::BenchmarkResponse::forbidden(&e),
    };

    serve_dashboard(&user)
}
// vuln-code-snippet end testcodeAuthnfailure041
