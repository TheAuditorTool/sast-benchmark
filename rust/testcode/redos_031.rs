//! CWE-1333: User-supplied pattern with ambiguous alternation (a+b|a+c)+d causes catastrophic backtracking.

use fancy_regex::Regex;

// vuln-code-snippet start testcodeRedos031
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_pattern = req.param("pattern");
    let text = req.param("text");

    let re = match Regex::new(&user_pattern) { // vuln-code-snippet target-line testcodeRedos031
        Ok(r) => r,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e.to_string()),
    };

    let found = re.is_match(&text).unwrap_or(false);
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos031
