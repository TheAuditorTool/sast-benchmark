//! CWE-614: Set-Cookie header assembled via string concatenation without security attributes.

// vuln-code-snippet start testcodeSecurecookie008
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let value = req.param("session");

    let header = format!("token={}; Path=/", value); // vuln-code-snippet target-line testcodeSecurecookie008

    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", header))
}
// vuln-code-snippet end testcodeSecurecookie008
