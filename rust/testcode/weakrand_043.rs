//! CWE-330: Function accepts a seed parameter but ignores it, always returning an OsRng token — secure.

// vuln-code-snippet start testcodeWeakrand043
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let seed: u64 = 1_700_000_000_123_456_789;
    let token = always_secure(seed); // vuln-code-snippet target-line testcodeWeakrand043

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn always_secure(_seed: u64) -> String {
    // Ignores caller-supplied seed; always draws from OS CSPRNG
    os_csprng_token()
}

fn os_csprng_token() -> String {
    // Simulates: rand::rngs::OsRng.gen::<[u8; 32]>() — OS CSPRNG
    let bytes: [u8; 32] = [0x2A; 32]; // placeholder for OsRng output
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
// vuln-code-snippet end testcodeWeakrand043
