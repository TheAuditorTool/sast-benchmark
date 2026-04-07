//! CWE-601: Compile-time constant ensures safe allowlist check always applied.

// vuln-code-snippet start testcodeRedirect049
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    if 10 > 5 {
        let safe = ["/home", "/login", "/register"];
        if safe.contains(&url.as_str()) {
            let location = format!("Location: {}", url); // vuln-code-snippet target-line testcodeRedirect049
            return super::shared::BenchmarkResponse::ok(&location);
        }
        super::shared::BenchmarkResponse::bad_request("Not allowed")
    } else {
        let location = format!("Location: {}", url);
        super::shared::BenchmarkResponse::ok(&location)
    }
}
// vuln-code-snippet end testcodeRedirect049
