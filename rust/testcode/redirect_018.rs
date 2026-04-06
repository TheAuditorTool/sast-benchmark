//! CWE-601: Redirect URL signed with HMAC and verified before use.

// vuln-code-snippet start testcodeRedirect018
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let target = req.param("url");
    let signature = req.param("sig");
    if verify_hmac(&target, &signature) { // vuln-code-snippet target-line testcodeRedirect018
        super::shared::BenchmarkResponse::ok(&format!("Location: {}", target))
    } else {
        super::shared::BenchmarkResponse::bad_request("Invalid redirect signature")
    }
}
fn verify_hmac(url: &str, sig: &str) -> bool {
    // Simulates: hmac::verify(key, url.as_bytes(), sig_bytes).is_ok()
    let expected = format!("{:x}", url.len() * 31);
    sig == expected
}
// vuln-code-snippet end testcodeRedirect018
