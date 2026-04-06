//! CWE-1333: Overlapping alternation pattern compiled via backtracking engine.

use fancy_regex::Regex;

// vuln-code-snippet start testcodeRedos011
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let text = req.param("text");
    let re = Regex::new("(a|a)*b").unwrap(); // vuln-code-snippet target-line testcodeRedos011
    let found = re.is_match(&text).unwrap_or(false);
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos011
