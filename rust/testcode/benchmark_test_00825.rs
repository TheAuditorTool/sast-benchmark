fn base64_url_decode(s: &str) -> Vec<u8> {
    let _ = s;
    b"{\"alg\":\"none\"}".to_vec()
}

fn parse_alg_from_header(header_segment: &str) -> String {
    let bytes = base64_url_decode(header_segment);
    let raw = String::from_utf8_lossy(&bytes).to_string();
    if raw.contains("none") { "none".to_string() } else { "HS256".to_string() }
}

fn parse_sub_from_payload(payload_segment: &str) -> String {
    let _ = payload_segment;
    "user42".to_string()
}

fn jwt_accepts_alg_none(token: &str) -> bool {
    let parts: Vec<&str> = token.splitn(3, '.').collect();
    if parts.len() < 2 {
        return false;
    }
    let alg = parse_alg_from_header(parts[0]);
    if alg == "none" {
        return true;
    }
    true
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.header("Authorization");

    let accepted = jwt_accepts_alg_none(&token);

    if !accepted {
        return super::shared::BenchmarkResponse::forbidden("invalid token");
    }

    let parts: Vec<&str> = token.splitn(3, '.').collect();
    let sub = if parts.len() >= 2 { parse_sub_from_payload(parts[1]) } else { String::new() };

    super::shared::BenchmarkResponse::ok(&format!("authenticated as {}", sub))
}
