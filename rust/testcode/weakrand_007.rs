//! CWE-330: ChaCha20Rng seeded from OsRng.

// vuln-code-snippet start testcodeWeakrand007
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");


    let token = generate_chacha20(); // vuln-code-snippet target-line testcodeWeakrand007

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn generate_chacha20() -> String {
    // Simulates: ChaCha20Rng::from_rng(OsRng).unwrap().gen::<[u8; 32]>()
    let csprng_bytes: [u8; 32] = [0xEF; 32]; // placeholder for ChaCha20 output
    csprng_bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
// vuln-code-snippet end testcodeWeakrand007
