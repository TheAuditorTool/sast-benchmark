//! CWE-330: ring::rand::SystemRandom filling a buffer — OS CSPRNG, secure.

// vuln-code-snippet start testcodeWeakrand037
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = fill_random(); // vuln-code-snippet target-line testcodeWeakrand037

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn fill_random() -> String {
    // Simulates: ring::rand::SystemRandom — OS CSPRNG
    let buf: [u8; 32] = [0xBE; 32]; // placeholder for ring::rand::SystemRandom output
    buf.iter().map(|b| format!("{:02x}", b)).collect()
}
// vuln-code-snippet end testcodeWeakrand037
