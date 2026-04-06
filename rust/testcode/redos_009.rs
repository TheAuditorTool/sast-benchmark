//! CWE-1333: Nested quantifiers (a+)+$ compiled via backtracking regex engine.

use fancy_regex::Regex;

// vuln-code-snippet start testcodeRedos009
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let text = req.param("text");
    let re = Regex::new("(a+)+$").unwrap(); // vuln-code-snippet target-line testcodeRedos009
    let found = re.is_match(&text).unwrap_or(false);
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos009
