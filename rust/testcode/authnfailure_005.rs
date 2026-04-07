//! CWE-287: JWT signature key not matched — any token with valid base64 structure is accepted.

struct Claims {
    pub sub: String,
}

fn jwt_decode_skip_sig(token: &str) -> String {
    // Stub: splits token, decodes header and payload, but skips signature verification entirely.
    // Any structurally valid JWT (three dot-separated base64 segments) passes.
    let parts: Vec<&str> = token.splitn(3, '.').collect();
    if parts.len() < 2 || parts[1].is_empty() {
        return String::new();
    }
    // Simulated payload decode — no HMAC key comparison.
    "forged_user".to_string()
}

// vuln-code-snippet start testcodeAuthnfailure005
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.header("Authorization");

    let subject = jwt_decode_skip_sig(&token); // vuln-code-snippet target-line testcodeAuthnfailure005

    if subject.is_empty() {
        return super::shared::BenchmarkResponse::forbidden("bad token format");
    }

    super::shared::BenchmarkResponse::ok(&format!("access granted to {}", subject))
}
// vuln-code-snippet end testcodeAuthnfailure005
