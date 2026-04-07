//! CWE-79: User input embedded in a JSON string that is then written into an inline script block.

// vuln-code-snippet start testcodeXss032
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_data = req.param("data");

    let json_val = format!("\"{}\"", user_data);
    let html = format!("<script>var config = {};</script>", json_val); // vuln-code-snippet target-line testcodeXss032

    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss032
