//! CWE-330: ChaCha20 seeded from OS entropy — CSPRNG, secure.

// vuln-code-snippet start testcodeWeakrand044
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = chacha20_token(); // vuln-code-snippet target-line testcodeWeakrand044

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn chacha20_token() -> String {
    // Simulates: ChaCha20Rng::from_entropy() — CSPRNG
    let bytes: [u8; 32] = [0x6D; 32]; // placeholder for ChaCha20Rng::from_entropy() output
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
// vuln-code-snippet end testcodeWeakrand044
