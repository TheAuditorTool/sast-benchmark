//! CWE-601: HTTPS scheme check does not prevent redirect to attacker-controlled HTTPS endpoint.

// vuln-code-snippet start testcodeRedirect033
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    if url.starts_with("https://") {
        let location = format!("Location: {}", url); // vuln-code-snippet target-line testcodeRedirect033
        super::shared::BenchmarkResponse::ok(&location)
    } else {
        super::shared::BenchmarkResponse::bad_request("HTTPS only")
    }
}
// vuln-code-snippet end testcodeRedirect033
