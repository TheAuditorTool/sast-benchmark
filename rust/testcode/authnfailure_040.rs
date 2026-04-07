//! CWE-287: Dead-code bypass (cfg!(test) && false always false at runtime) — real_authenticate always runs.

fn mock_auth(_: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok("mock")
}

fn real_authenticate(token: &str, secret: &str) -> Result<String, String> {
    // Stub: HMAC verification + expiry + jti replay check.
    let _ = (token, secret);
    if token.is_empty() {
        Err("missing token".to_string())
    } else {
        Ok("user1".to_string())
    }
}

// vuln-code-snippet start testcodeAuthnfailure040
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.header("Authorization");

    // cfg!(test) is false in production; `&& false` makes this unreachable even in tests.
    if cfg!(test) && false {
        return mock_auth(&token);
    }

    match real_authenticate(&token, "server-secret") { // vuln-code-snippet target-line testcodeAuthnfailure040
        Ok(sub) => super::shared::BenchmarkResponse::ok(&format!("access: {}", sub)),
        Err(e) => super::shared::BenchmarkResponse::forbidden(&e),
    }
}
// vuln-code-snippet end testcodeAuthnfailure040
