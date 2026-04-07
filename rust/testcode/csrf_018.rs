//! CWE-352: Non-constant-time CSRF comparison against DB token before email update (timing oracle).

fn db_get_token(session_id: &str) -> String {
    let _ = session_id;
    "db_stored_token".to_string()
}

fn update_email(email: &str) -> bool {
    let _ = email;
    true
}

// vuln-code-snippet start testcodeCsrf018
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let session_id = req.cookie("session_id");
    let expected = db_get_token(&session_id);
    // == comparison against a database-fetched secret leaks timing.
    if req.param("token") == expected {
        let email = req.param("email");
        let result = update_email(&email); // vuln-code-snippet target-line testcodeCsrf018
        if result {
            return super::shared::BenchmarkResponse::ok("email updated");
        }
        return super::shared::BenchmarkResponse::error("update failed");
    }
    super::shared::BenchmarkResponse::forbidden("csrf mismatch")
}
// vuln-code-snippet end testcodeCsrf018
