//! CWE-1333: Version-string pattern with polynomial catastrophic backtracking.

use fancy_regex::Regex;

// vuln-code-snippet start testcodeRedos013
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let text = req.param("version");
    let re = Regex::new(r"(\d+\.)+\d+").unwrap(); // vuln-code-snippet target-line testcodeRedos013
    let found = re.is_match(&text).unwrap_or(false);
    super::shared::BenchmarkResponse::ok(&format!("Valid version: {}", found))
}
// vuln-code-snippet end testcodeRedos013
