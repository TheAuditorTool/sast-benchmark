//! CWE-330: Helper generates OsRng-backed token stored in struct field and returned — secure.

// vuln-code-snippet start testcodeWeakrand040
struct AuthResponse {
    token: String,
}

fn gen_secure_token() -> String {
    // Simulates: rand::rngs::OsRng.gen::<[u8; 32]>() — OS CSPRNG
    let bytes: [u8; 32] = [0x5C; 32]; // placeholder for OsRng output
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let resp = AuthResponse { token: gen_secure_token() }; // vuln-code-snippet target-line testcodeWeakrand040

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", resp.token))
}
// vuln-code-snippet end testcodeWeakrand040
