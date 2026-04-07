//! CWE-287: Dead-code bypass ("prod" == "debug" always false) — verify_jwt always executed.

fn skip_auth(_: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok("skipped")
}

fn verify_jwt(token: &str, secret: &str) -> Result<String, String> {
    // Stub: HMAC verification + expiry check.
    let _ = (token, secret);
    if token.len() > 10 {
        Ok("user1".to_string())
    } else {
        Err("invalid token".to_string())
    }
}

// vuln-code-snippet start testcodeAuthnfailure038
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.header("Authorization");

    // String literal comparison is always false; dead branch never executes.
    if "prod" == "debug" {
        return skip_auth("never reached");
    }

    match verify_jwt(&token, "server-secret") { // vuln-code-snippet target-line testcodeAuthnfailure038
        Ok(sub) => super::shared::BenchmarkResponse::ok(&format!("authenticated: {}", sub)),
        Err(e) => super::shared::BenchmarkResponse::forbidden(&e),
    }
}
// vuln-code-snippet end testcodeAuthnfailure038
