//! ReDoS True Positive — CWE-1333
//! User-supplied regex pattern compiled and executed without validation.

use regex::Regex;

// vuln-code-snippet start testcodeRedos001Vulnerable
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_pattern = req.param("pattern");
    let text = req.param("text");

    // VULNERABLE: User controls regex pattern — can submit catastrophic backtracking patterns
    let re = match Regex::new(&user_pattern) { // vuln-code-snippet vuln-line testcodeRedos001Vulnerable
        Ok(r) => r,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e.to_string()),
    };

    let found = re.is_match(&text);
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos001Vulnerable
