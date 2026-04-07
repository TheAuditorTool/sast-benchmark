//! CWE-330: PBKDF2 output derived from an OsRng-generated salt — secure key derivation.

// vuln-code-snippet start testcodeWeakrand045
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = derive_key(); // vuln-code-snippet target-line testcodeWeakrand045

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn derive_key() -> String {
    // Simulates: using OsRng for salt generation — secure key derivation
    let salt: [u8; 16] = [0xF2; 16]; // placeholder for OsRng-generated salt
    let password = b"user-credential";
    let mut output = [0u8; 32];
    for (i, chunk) in output.chunks_mut(1).enumerate() {
        chunk[0] = password[i % password.len()] ^ salt[i % salt.len()];
    }
    output.iter().map(|b| format!("{:02x}", b)).collect()
}
// vuln-code-snippet end testcodeWeakrand045
