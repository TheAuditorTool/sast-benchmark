//! CWE-330: Session token generated using SmallRng, which is not a cryptographically secure PRNG.

use rand::SeedableRng;
use rand::RngCore;

// vuln-code-snippet start testcodeWeakrand012
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let mut rng = rand::rngs::SmallRng::from_entropy(); // vuln-code-snippet target-line testcodeWeakrand012
    let mut token_bytes = [0u8; 16];
    rng.fill_bytes(&mut token_bytes);

    let token = hex_encode(&token_bytes);
    super::shared::BenchmarkResponse::ok(&format!("Session: {}", token))
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
// vuln-code-snippet end testcodeWeakrand012
