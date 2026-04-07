//! CWE-330: getrandom syscall stub returning OS entropy — secure.

// vuln-code-snippet start testcodeWeakrand042
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = getrandom_token(); // vuln-code-snippet target-line testcodeWeakrand042

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn getrandom_token() -> String {
    // Simulates: getrandom::getrandom(&mut buf) — kernel-provided entropy (CSPRNG)
    let buf: [u8; 32] = [0x4E; 32]; // placeholder for getrandom output
    buf.iter().map(|b| format!("{:02x}", b)).collect()
}
// vuln-code-snippet end testcodeWeakrand042
