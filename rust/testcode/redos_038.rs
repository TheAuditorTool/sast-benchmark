//! CWE-1333: User pattern validated to alphanumeric-only before fancy_regex compile — structural validation prevents catastrophic patterns.

use fancy_regex::Regex;

// vuln-code-snippet start testcodeRedos038
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_pattern = req.param("pattern");
    let text = req.param("text");

    if !user_pattern.chars().all(|c| c.is_alphanumeric()) {
        return super::shared::BenchmarkResponse::bad_request("Pattern must be alphanumeric only");
    }

    let re = match Regex::new(&user_pattern) { // vuln-code-snippet target-line testcodeRedos038
        Ok(r) => r,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e.to_string()),
    };

    let found = re.is_match(&text).unwrap_or(false);
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos038
