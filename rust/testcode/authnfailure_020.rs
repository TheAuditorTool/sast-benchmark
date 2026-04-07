//! CWE-287: X-Auth-Level header value trusted to grant elevated access without server-side auth.

fn elevated_access(username: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("elevated access for {}", username))
}

fn standard_access(username: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("standard access for {}", username))
}

fn verify_session_token(token: &str) -> Option<String> {
    // Stub: returns username from valid session.
    if token.is_empty() { None } else { Some("bob".to_string()) }
}

// vuln-code-snippet start testcodeAuthnfailure020
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let session = req.cookie("session");

    let username = match verify_session_token(&session) {
        Some(u) => u,
        None => return super::shared::BenchmarkResponse::forbidden("invalid session"),
    };

    // Privilege level comes from an attacker-controlled request header.
    if req.header("X-Auth-Level") == "admin" {
        return elevated_access(&username); // vuln-code-snippet target-line testcodeAuthnfailure020
    }

    standard_access(&username)
}
// vuln-code-snippet end testcodeAuthnfailure020
