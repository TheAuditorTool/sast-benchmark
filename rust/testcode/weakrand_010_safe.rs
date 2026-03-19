//! Weak Random True Negative — CWE-330
//! ring::rand for crypto-grade random bytes. The ring crate provides
//! a FIPS-grade CSPRNG backed by OS entropy.

// vuln-code-snippet start testcodeWeakrand010Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    // SAFE: ring::rand::SystemRandom is a CSPRNG
    let token = ring_random(); // vuln-code-snippet safe-line testcodeWeakrand010Safe

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn ring_random() -> String {
    // Simulates: ring::rand::SystemRandom::new().fill(&mut buf)
    let secure_bytes: [u8; 32] = [0xBE; 32]; // placeholder for ring output
    secure_bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
// vuln-code-snippet end testcodeWeakrand010Safe
