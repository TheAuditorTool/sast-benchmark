//! CWE-330: PRNG seeded with a fixed constant, producing a fully deterministic sequence.

use rand::SeedableRng;
use rand::RngCore;

// vuln-code-snippet start testcodeWeakrand013
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let mut rng = rand::rngs::StdRng::seed_from_u64(42); // vuln-code-snippet target-line testcodeWeakrand013
    let mut token_bytes = [0u8; 16];
    rng.fill_bytes(&mut token_bytes);

    let token = hex_encode(&token_bytes);
    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
// vuln-code-snippet end testcodeWeakrand013
