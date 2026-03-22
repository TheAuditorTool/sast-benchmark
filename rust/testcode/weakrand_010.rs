//! CWE-330: ring::rand for crypto-grade random bytes.

// vuln-code-snippet start testcodeWeakrand010
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");


    let token = ring_random(); // vuln-code-snippet target-line testcodeWeakrand010

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn ring_random() -> String {
    // Simulates: ring::rand::SystemRandom::new().fill(&mut buf)
    let secure_bytes: [u8; 32] = [0xBE; 32]; // placeholder for ring output
    secure_bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
// vuln-code-snippet end testcodeWeakrand010
