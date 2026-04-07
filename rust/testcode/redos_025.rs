//! CWE-1333: User-supplied pattern with repeated wildcard (.*a){10} causes exponential backtracking.

use fancy_regex::Regex;

// vuln-code-snippet start testcodeRedos025
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_pattern = req.param("pattern");
    let text = req.param("text");

    let re = match Regex::new(&user_pattern) { // vuln-code-snippet target-line testcodeRedos025
        Ok(r) => r,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e.to_string()),
    };

    let found = re.is_match(&text).unwrap_or(false);
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos025
