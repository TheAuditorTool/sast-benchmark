//! CWE-287: bcrypt_verify result exhaustively matched — every arm handled, wrong password always forbidden.

fn bcrypt_verify(password: &str, hash: &str) -> Result<bool, String> {
    // Stub: constant-time bcrypt comparison.
    let _ = (password, hash);
    Ok(password == "correct_password")
}

fn get_hash(username: &str) -> Option<String> {
    let _ = username;
    Some("$2b$12$fakehash".to_string())
}

fn proceed(username: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("logged in: {}", username))
}

// vuln-code-snippet start testcodeAuthnfailure032
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let pw = req.param("password");

    let hash = match get_hash(&username) {
        Some(h) => h,
        None => return super::shared::BenchmarkResponse::forbidden("unknown user"),
    };

    match bcrypt_verify(&pw, &hash) { // vuln-code-snippet target-line testcodeAuthnfailure032
        Ok(true) => proceed(&username),
        Ok(false) => super::shared::BenchmarkResponse::forbidden("wrong password"),
        Err(_) => super::shared::BenchmarkResponse::error("internal error"),
    }
}
// vuln-code-snippet end testcodeAuthnfailure032
