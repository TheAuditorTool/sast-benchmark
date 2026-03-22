//! CWE-330: getrandom crate (simulated) for secure random bytes.

// vuln-code-snippet start testcodeWeakrand009
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");


    let token = getrandom_bytes(); // vuln-code-snippet target-line testcodeWeakrand009

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn getrandom_bytes() -> String {
    // Simulates: getrandom::getrandom(&mut buf).unwrap()
    let mut buf = [0u8; 32];
    buf.iter_mut().enumerate().for_each(|(i, b)| *b = (i as u8).wrapping_add(0xA0));
    buf.iter().map(|b| format!("{:02x}", b)).collect()
}
// vuln-code-snippet end testcodeWeakrand009
