//! CWE-287: Session token compared with == (non-constant-time) — susceptible to timing side-channel.

fn get_stored_session_token(user_id: &str) -> String {
    // Stub: retrieves the server-side session token for the given user.
    let _ = user_id;
    "a3f9b2c1d4e5f6a7b8c9d0e1f2a3b4c5".to_string()
}

fn grant_access(user_id: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("session valid for {}", user_id))
}

// vuln-code-snippet start testcodeAuthnfailure021
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_id = req.param("user_id");
    let session_token = req.cookie("session_token");

    let stored_token = get_stored_session_token(&user_id);

    // Standard == on String performs short-circuit comparison; first differing byte leaks timing info.
    if session_token == stored_token { // vuln-code-snippet target-line testcodeAuthnfailure021
        grant_access(&user_id)
    } else {
        super::shared::BenchmarkResponse::forbidden("invalid session")
    }
}
// vuln-code-snippet end testcodeAuthnfailure021
