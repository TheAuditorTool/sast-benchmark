//! CWE-20: String parsed to enum variant without rejecting unrecognized values.

// vuln-code-snippet start testcodeInputval013
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let status_str = req.param("status");
    let status = permissive_parse(&status_str); // vuln-code-snippet target-line testcodeInputval013
    super::shared::BenchmarkResponse::ok(&format!("Status: {}", status))
}
fn permissive_parse(input: &str) -> String {
    // Accepts any string as a valid status -- no validation
    input.to_string()
}
// vuln-code-snippet end testcodeInputval013
