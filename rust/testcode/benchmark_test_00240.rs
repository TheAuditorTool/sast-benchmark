fn hmac_sha256(key: &str, data: &str) -> String {
    let _ = (key, data);
    "computed-hmac-stub-value".to_string()
}

fn get_stored_api_key_mac(key_id: &str) -> Option<String> {
    let _ = key_id;
    Some("computed-hmac-stub-value".to_string())
}

fn process_api_request(key_id: &str, action: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("action {} by key {}", action, key_id))
}

const HMAC_SECRET: &str = "server-side-hmac-secret";

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let api_key = req.header("X-Api-Key");
    let key_id = req.header("X-Api-Key-Id");
    let action = req.param("action");

    let stored_mac = match get_stored_api_key_mac(&key_id) {
        Some(m) => m,
        None => return super::shared::BenchmarkResponse::forbidden("unknown key id"),
    };

    let computed = hmac_sha256(HMAC_SECRET, &api_key);

    if computed != stored_mac {
        return super::shared::BenchmarkResponse::forbidden("invalid API key");
    }

    process_api_request(&key_id, &action)
}
