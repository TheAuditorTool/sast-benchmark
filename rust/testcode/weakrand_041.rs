//! CWE-330: HashMap holds both weak and strong tokens; response uses the "token" key (OsRng) — secure.

use std::collections::HashMap;

// vuln-code-snippet start testcodeWeakrand041
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let mut m: HashMap<&str, String> = HashMap::new();
    m.insert("weak", timestamp_token());
    m.insert("token", os_random_token());

    let tok = m.get("token").unwrap(); // vuln-code-snippet target-line testcodeWeakrand041

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", tok))
}

fn timestamp_token() -> String {
    // Simulates: SystemTime::now().duration_since(EPOCH).as_nanos() as u64
    let val: u64 = 1_700_000_000_123_456_789;
    format!("{:016x}", val)
}

fn os_random_token() -> String {
    // Simulates: rand::rngs::OsRng.gen::<[u8; 32]>() — OS CSPRNG
    let bytes: [u8; 32] = [0x7B; 32];
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
// vuln-code-snippet end testcodeWeakrand041
