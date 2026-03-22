//! CWE-330: OsRng for token generation via OS entropy pool.

// vuln-code-snippet start testcodeWeakrand005
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");


    let token = generate_with_os_rng(); // vuln-code-snippet target-line testcodeWeakrand005

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn generate_with_os_rng() -> String {
    // Simulates: use rand::rngs::OsRng; OsRng.gen::<[u8; 32]>()
    let secure_bytes: [u8; 32] = [0xAB; 32]; // placeholder for OsRng output
    secure_bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
// vuln-code-snippet end testcodeWeakrand005
