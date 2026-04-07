//! CWE-1333: User-supplied pattern with catastrophic repetition ([a-zA-Z]+)* compiled by backtracking engine.

use fancy_regex::Regex;

// vuln-code-snippet start testcodeRedos027
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_pattern = req.param("pattern");
    let text = req.param("text");

    let re = match Regex::new(&user_pattern) { // vuln-code-snippet target-line testcodeRedos027
        Ok(r) => r,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e.to_string()),
    };

    let found = re.is_match(&text).unwrap_or(false);
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos027
