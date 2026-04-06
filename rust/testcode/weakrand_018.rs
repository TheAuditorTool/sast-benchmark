//! CWE-330: Nonce generated using ring SystemRandom backed by OS CSPRNG.

// vuln-code-snippet start testcodeWeakrand018
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _op = req.param("op");
    // Simulates: ring::rand::SystemRandom::new()
    let nonce = ring_system_random_nonce(); // vuln-code-snippet target-line testcodeWeakrand018
    super::shared::BenchmarkResponse::ok(&format!("Nonce: {:x?}", &nonce[..4]))
}
fn ring_system_random_nonce() -> [u8; 12] {
    // Simulates: ring::rand::generate::<[u8; 12]>(&SystemRandom::new())
    [0xABu8; 12]
}
// vuln-code-snippet end testcodeWeakrand018
