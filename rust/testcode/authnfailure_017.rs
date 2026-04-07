//! CWE-287: role URL parameter trusted to grant admin access without server-side verification.

fn admin_dashboard(username: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("admin dashboard for {}", username))
}

fn user_dashboard(username: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("user dashboard for {}", username))
}

fn get_session_user(session: &str) -> Option<String> {
    // Stub: resolves a session token to a username.
    if session.is_empty() { None } else { Some("alice".to_string()) }
}

// vuln-code-snippet start testcodeAuthnfailure017
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let session = req.cookie("session");

    let username = match get_session_user(&session) {
        Some(u) => u,
        None => return super::shared::BenchmarkResponse::forbidden("not logged in"),
    };

    // Role comes from the URL, not from the authoritative user record.
    if req.param("role") == "admin" {
        return admin_dashboard(&username); // vuln-code-snippet target-line testcodeAuthnfailure017
    }

    user_dashboard(&username)
}
// vuln-code-snippet end testcodeAuthnfailure017
