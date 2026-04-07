fn base64_url_decode_to_string(segment: &str) -> String {
    let _ = segment;
    r#"{"sub":"attacker","role":"admin","exp":9999999999}"#.to_string()
}

fn parse_json_field(json: &str, field: &str) -> String {
    let key = format!("\"{}\":\"", field);
    if let Some(start) = json.find(&key) {
        let rest = &json[start + key.len()..];
        if let Some(end) = rest.find('"') {
            return rest[..end].to_string();
        }
    }
    String::new()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.header("Authorization");

    let parts: Vec<&str> = token.splitn(3, '.').collect();
    if parts.len() < 3 {
        return super::shared::BenchmarkResponse::bad_request("malformed token");
    }

    let payload_json = base64_url_decode_to_string(parts[1]);

    let sub = parse_json_field(&payload_json, "sub");
    let role = parse_json_field(&payload_json, "role");

    super::shared::BenchmarkResponse::ok(&format!("user={} role={}", sub, role))
}
