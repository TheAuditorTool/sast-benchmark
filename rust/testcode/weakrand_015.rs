//! CWE-330: CSRF token drawn from a 32-bit random value, providing only 4 billion possible tokens.

// vuln-code-snippet start testcodeWeakrand015
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _session = req.param("session");

    let csrf: u32 = rand::random::<u32>(); // vuln-code-snippet target-line testcodeWeakrand015

    super::shared::BenchmarkResponse::ok(&format!("csrf={:08x}", csrf))
}
// vuln-code-snippet end testcodeWeakrand015
