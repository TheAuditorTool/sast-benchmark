//! CWE-1333: User pattern compiled with regex crate RE2 engine guaranteeing linear time.

use regex::Regex;

// vuln-code-snippet start testcodeRedos016
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_pattern = req.param("pattern");
    let text = req.param("text");
    let re = match Regex::new(&user_pattern) { // vuln-code-snippet target-line testcodeRedos016
        Ok(r) => r,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e.to_string()),
    };
    let found = re.is_match(&text);
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos016
