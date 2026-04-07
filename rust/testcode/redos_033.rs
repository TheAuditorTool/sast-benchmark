//! CWE-1333: Length-only guard (< 100) does not eliminate catastrophic nested quantifiers in user pattern.

use fancy_regex::Regex;

// vuln-code-snippet start testcodeRedos033
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_pattern = req.param("pattern");
    let text = req.param("text");

    if user_pattern.len() >= 100 {
        return super::shared::BenchmarkResponse::bad_request("Pattern too long");
    }

    let re = match Regex::new(&user_pattern) { // vuln-code-snippet target-line testcodeRedos033
        Ok(r) => r,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e.to_string()),
    };

    let found = re.is_match(&text).unwrap_or(false);
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos033
