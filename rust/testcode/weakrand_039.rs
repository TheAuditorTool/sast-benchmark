//! CWE-330: 32-byte hex token generated from OsRng — OS CSPRNG, secure.

// vuln-code-snippet start testcodeWeakrand039
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = os_random_hex(); // vuln-code-snippet target-line testcodeWeakrand039

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn os_random_hex() -> String {
    // Simulates: rand::rngs::OsRng.gen::<[u8; 32]>() — OS CSPRNG
    let bytes: [u8; 32] = [0x9F; 32]; // placeholder for OsRng output
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
// vuln-code-snippet end testcodeWeakrand039
