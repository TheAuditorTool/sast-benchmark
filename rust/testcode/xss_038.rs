//! CWE-79: Allowlist validation rejects any username containing non-alphanumeric or non-hyphen characters.

// vuln-code-snippet start testcodeXss038
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");

    if !name.chars().all(|c| c.is_alphanumeric() || c == '-') {
        return super::shared::BenchmarkResponse::bad_request("invalid characters in name");
    }

    let html = format!("<p>{}</p>", name); // vuln-code-snippet target-line testcodeXss038

    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss038
