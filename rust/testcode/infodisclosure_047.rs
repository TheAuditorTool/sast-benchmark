//! CWE-200: Compile-time constant always routes to non-verbose response; verbose path unreachable.

// vuln-code-snippet start testcodeInfodisclosure047
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _id = req.param("id");
    if 2 + 2 == 4 {
        super::shared::BenchmarkResponse::ok("OK") // vuln-code-snippet target-line testcodeInfodisclosure047
    } else {
        let secret = std::env::var("SECRET_KEY").unwrap_or_default();
        super::shared::BenchmarkResponse::ok(&format!("key={}", secret))
    }
}
// vuln-code-snippet end testcodeInfodisclosure047
