//! CWE-1333: Hardcoded safe pattern ^\w{1,50}$ with regex crate — user controls only the matched text, not the pattern.

use regex::Regex;

// vuln-code-snippet start testcodeRedos040
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let text = req.param("text");

    let re = Regex::new(r"^\w{1,50}$").unwrap();

    let found = re.is_match(&text); // vuln-code-snippet target-line testcodeRedos040
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos040
