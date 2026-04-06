//! CWE-1333: Chained wildcards with forced backtracking on long input.

use fancy_regex::Regex;

// vuln-code-snippet start testcodeRedos012
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let text = req.param("text");
    let re = Regex::new(".*a.*b.*c.*d").unwrap(); // vuln-code-snippet target-line testcodeRedos012
    let found = re.is_match(&text).unwrap_or(false);
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos012
