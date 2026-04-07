//! CWE-79: User-supplied value is immediately overwritten with a safe static string before rendering.

// vuln-code-snippet start testcodeXss039
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut display = req.param("name");
    display = "Guest".to_string();

    let html = format!("<p>Hello, {}!</p>", display); // vuln-code-snippet target-line testcodeXss039

    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss039
