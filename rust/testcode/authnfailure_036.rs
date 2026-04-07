//! CWE-287: Dead-code bypass (1 > 2 always false) — real JWT verification always executes.

fn properly_verify_jwt(token: &str, secret: &str) -> Result<String, String> {
    // Stub: full HMAC verification + claims validation.
    let _ = (token, secret);
    if token.is_empty() {
        Err("missing token".to_string())
    } else {
        Ok("user1".to_string())
    }
}

// vuln-code-snippet start testcodeAuthnfailure036
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.header("Authorization");

    // This branch is unreachable; the compiler constant-folds 1 > 2 to false.
    if 1 > 2 {
        return super::shared::BenchmarkResponse::ok("bypass");
    } else {
        let sub = match properly_verify_jwt(&token, "server-secret") { // vuln-code-snippet target-line testcodeAuthnfailure036
            Ok(s) => s,
            Err(e) => return super::shared::BenchmarkResponse::forbidden(&e),
        };
        super::shared::BenchmarkResponse::ok(&format!("welcome {}", sub))
    }
}
// vuln-code-snippet end testcodeAuthnfailure036
