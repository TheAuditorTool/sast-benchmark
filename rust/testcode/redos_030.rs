//! CWE-1333: Length check < 20 does not prevent catastrophic backtracking from nested quantifiers.

use fancy_regex::Regex;

// vuln-code-snippet start testcodeRedos030
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_pattern = req.param("pattern");
    let text = req.param("text");

    if user_pattern.len() >= 20 {
        return super::shared::BenchmarkResponse::bad_request("Pattern too long");
    }

    let re = match Regex::new(&user_pattern) { // vuln-code-snippet target-line testcodeRedos030
        Ok(r) => r,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e.to_string()),
    };

    let found = re.is_match(&text).unwrap_or(false);
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos030
