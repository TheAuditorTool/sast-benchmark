//! CWE-330: OsRng-generated token stored in a struct field and returned to user — secure.

// vuln-code-snippet start testcodeWeakrand035
struct SessionToken {
    value: String,
}

impl SessionToken {
    fn new() -> Self {
        // Simulates: rand::rngs::OsRng.gen::<[u8; 32]>() — OS CSPRNG
        let bytes: [u8; 32] = [0xC7; 32]; // placeholder for OsRng output
        let value = bytes.iter().map(|b| format!("{:02x}", b)).collect();
        SessionToken { value }
    }
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let tok = SessionToken::new();
    let token = tok.value.clone(); // vuln-code-snippet target-line testcodeWeakrand035

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}
// vuln-code-snippet end testcodeWeakrand035
