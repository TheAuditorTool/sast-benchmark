//! CWE-1333: User part interpolated via format! into a catastrophic wrapping pattern, compiled by backtracking engine.

use fancy_regex::Regex;

// vuln-code-snippet start testcodeRedos029
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let part = req.param("part");
    let text = req.param("text");

    let pat = format!("({})+", part);

    let re = match Regex::new(&pat) { // vuln-code-snippet target-line testcodeRedos029
        Ok(r) => r,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e.to_string()),
    };

    let found = re.is_match(&text).unwrap_or(false);
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos029
