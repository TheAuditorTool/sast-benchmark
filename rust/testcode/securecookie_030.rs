//! CWE-614: User preference cookie containing sensitive identifier lacks security flags.

// vuln-code-snippet start testcodeSecurecookie030
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let account_id = req.param("account_id");
    let pref_cookie = format!("account={}", account_id); // vuln-code-snippet target-line testcodeSecurecookie030
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", pref_cookie))
}
// vuln-code-snippet end testcodeSecurecookie030
