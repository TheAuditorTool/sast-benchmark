//! CWE-287: JWT with alg=none accepted — algorithm field from token trusted, no signature required.

fn base64_url_decode(s: &str) -> Vec<u8> {
    // Stub base64url decode.
    let _ = s;
    b"{\"alg\":\"none\"}".to_vec()
}

fn parse_alg_from_header(header_segment: &str) -> String {
    let bytes = base64_url_decode(header_segment);
    let raw = String::from_utf8_lossy(&bytes).to_string();
    // Stub: extract alg value from JSON header.
    if raw.contains("none") { "none".to_string() } else { "HS256".to_string() }
}

fn parse_sub_from_payload(payload_segment: &str) -> String {
    let _ = payload_segment;
    // Stub: extract sub from base64-decoded payload JSON.
    "user42".to_string()
}

fn jwt_accepts_alg_none(token: &str) -> bool {
    let parts: Vec<&str> = token.splitn(3, '.').collect();
    if parts.len() < 2 {
        return false;
    }
    let alg = parse_alg_from_header(parts[0]);
    // Vulnerability: alg=none means no signature is present or checked.
    if alg == "none" {
        return true;
    }
    // For other algorithms a real signature check would go here — stub returns true.
    true
}

// vuln-code-snippet start testcodeAuthnfailure002
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.header("Authorization");

    let accepted = jwt_accepts_alg_none(&token); // vuln-code-snippet target-line testcodeAuthnfailure002

    if !accepted {
        return super::shared::BenchmarkResponse::forbidden("invalid token");
    }

    let parts: Vec<&str> = token.splitn(3, '.').collect();
    let sub = if parts.len() >= 2 { parse_sub_from_payload(parts[1]) } else { String::new() };

    super::shared::BenchmarkResponse::ok(&format!("authenticated as {}", sub))
}
// vuln-code-snippet end testcodeAuthnfailure002
