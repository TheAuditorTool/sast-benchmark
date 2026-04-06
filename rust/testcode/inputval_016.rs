//! CWE-20: Date string parsed without format validation, accepting invalid dates.

// vuln-code-snippet start testcodeInputval016
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let date_str = req.param("date");
    let date = permissive_date_parse(&date_str); // vuln-code-snippet target-line testcodeInputval016
    super::shared::BenchmarkResponse::ok(&format!("Date: {}", date))
}
fn permissive_date_parse(input: &str) -> String {
    // Accepts any string as date -- no format check
    input.to_string()
}
// vuln-code-snippet end testcodeInputval016
