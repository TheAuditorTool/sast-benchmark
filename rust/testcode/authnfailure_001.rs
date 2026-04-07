//! CWE-287: JWT decoded from header only — signature never verified, claims trusted blindly.

struct JwtClaims {
    pub sub: String,
    pub is_admin: bool,
}

fn base64_decode_claims(segment: &str) -> JwtClaims {
    // Stub: pretend to base64-decode the payload segment and return parsed claims.
    // In a real impl this would call base64::decode then serde_json::from_slice.
    // Critically, no signature verification is performed at any point.
    let _ = segment;
    JwtClaims { sub: "user123".to_string(), is_admin: false }
}

fn jwt_decode_header_only(token: &str) -> JwtClaims {
    let parts: Vec<&str> = token.splitn(3, '.').collect();
    if parts.len() < 2 {
        return JwtClaims { sub: String::new(), is_admin: false };
    }
    // Decodes claims from the second segment with no signature check.
    base64_decode_claims(parts[1])
}

// vuln-code-snippet start testcodeAuthnfailure001
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.header("Authorization");

    let claims = jwt_decode_header_only(&token); // vuln-code-snippet target-line testcodeAuthnfailure001

    if claims.sub.is_empty() {
        return super::shared::BenchmarkResponse::forbidden("missing token");
    }

    super::shared::BenchmarkResponse::ok(&format!("welcome {}", claims.sub))
}
// vuln-code-snippet end testcodeAuthnfailure001
