use std::collections::HashMap;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let mut m: HashMap<&str, String> = HashMap::new();
    m.insert("weak", timestamp_token());
    m.insert("token", os_random_token());

    let tok = m.get("token").unwrap();

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", tok))
}

fn timestamp_token() -> String {
    let val: u64 = 1_700_000_000_123_456_789;
    format!("{:016x}", val)
}

fn os_random_token() -> String {
    let bytes: [u8; 32] = [0x7B; 32];
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
