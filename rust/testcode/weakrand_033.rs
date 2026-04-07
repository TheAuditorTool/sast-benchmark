//! CWE-330: OsRng generating 32 random bytes — OS CSPRNG, secure.

// vuln-code-snippet start testcodeWeakrand033
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = os_random_bytes(); // vuln-code-snippet target-line testcodeWeakrand033

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn os_random_bytes() -> String {
    // Simulates: rand::rngs::OsRng.gen::<[u8; 32]>() — OS CSPRNG
    let bytes: [u8; 32] = [0xA3; 32]; // placeholder for OsRng output
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
// vuln-code-snippet end testcodeWeakrand033
