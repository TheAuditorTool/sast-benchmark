//! Cross-Site Scripting True Negative — CWE-79
//! Response served as application/json. Browsers will not render HTML
//! or execute scripts in JSON content type responses.

// vuln-code-snippet start testcodeXss007Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_input = req.param("search");

    // SAFE: Content-Type: application/json — browser will not render HTML
    let json_body = format!("{{\"query\":\"{}\",\"results\":[]}}", user_input); // vuln-code-snippet safe-line testcodeXss007Safe

    let mut resp = super::shared::BenchmarkResponse::ok(&json_body);
    resp.body = format!("Content-Type: application/json\n\n{}", json_body);
    resp
}
// vuln-code-snippet end testcodeXss007Safe
