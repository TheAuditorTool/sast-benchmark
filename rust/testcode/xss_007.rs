//! CWE-79: Response served as application/json.

// vuln-code-snippet start testcodeXss007
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_input = req.param("search");

    let json_body = format!("{{\"query\":\"{}\",\"results\":[]}}", user_input); // vuln-code-snippet target-line testcodeXss007

    let mut resp = super::shared::BenchmarkResponse::ok(&json_body);
    resp.body = format!("Content-Type: application/json\n\n{}", json_body);
    resp
}
// vuln-code-snippet end testcodeXss007
