//! CWE-287: JWT claims extracted by splitting on '.' and base64-decoding middle segment — no signature verification.

fn base64_url_decode_to_string(segment: &str) -> String {
    // Stub: simulate base64url decode. Returns fake JSON payload.
    let _ = segment;
    r#"{"sub":"attacker","role":"admin","exp":9999999999}"#.to_string()
}

fn parse_json_field(json: &str, field: &str) -> String {
    // Stub: extract a string field from a JSON object without a real parser.
    let key = format!("\"{}\":\"", field);
    if let Some(start) = json.find(&key) {
        let rest = &json[start + key.len()..];
        if let Some(end) = rest.find('"') {
            return rest[..end].to_string();
        }
    }
    String::new()
}

// vuln-code-snippet start testcodeAuthnfailure003
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.header("Authorization");

    let parts: Vec<&str> = token.splitn(3, '.').collect();
    if parts.len() < 3 {
        return super::shared::BenchmarkResponse::bad_request("malformed token");
    }

    // Decode payload without verifying the signature in parts[2].
    let payload_json = base64_url_decode_to_string(parts[1]); // vuln-code-snippet target-line testcodeAuthnfailure003

    let sub = parse_json_field(&payload_json, "sub");
    let role = parse_json_field(&payload_json, "role");

    super::shared::BenchmarkResponse::ok(&format!("user={} role={}", sub, role))
}
// vuln-code-snippet end testcodeAuthnfailure003
