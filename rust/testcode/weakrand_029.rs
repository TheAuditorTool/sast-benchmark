//! CWE-330: SmallRng-like PRNG struct seeded from timestamp stored in a struct field.

// vuln-code-snippet start testcodeWeakrand029
struct WeakRng {
    state: u64,
}

impl WeakRng {
    fn next(&mut self) -> u64 {
        // Simulates: SmallRng internal step — xorshift64
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    // Simulates: SmallRng::seed_from_u64(SystemTime timestamp)
    let mut rng = WeakRng { state: 1_700_000_000_123_456_789 };
    let token = generate_token(&mut rng); // vuln-code-snippet target-line testcodeWeakrand029

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn generate_token(rng: &mut WeakRng) -> String {
    format!("{:016x}", rng.next())
}
// vuln-code-snippet end testcodeWeakrand029
