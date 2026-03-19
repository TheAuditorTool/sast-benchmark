//! Weak Random True Positive — CWE-330
//! thread_rng used for auth token generation. thread_rng is not
//! cryptographically secure — predictable output in security context.

// vuln-code-snippet start testcodeWeakrand001Vulnerable
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    // VULNERABLE: thread_rng is not CSPRNG — predictable auth tokens
    let token = generate_with_thread_rng(); // vuln-code-snippet vuln-line testcodeWeakrand001Vulnerable

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn generate_with_thread_rng() -> String {
    // Simulates: rand::thread_rng().gen::<u64>()
    let pseudo: u64 = 0xDEADBEEF12345678;
    format!("{:016x}", pseudo)
}
// vuln-code-snippet end testcodeWeakrand001Vulnerable
