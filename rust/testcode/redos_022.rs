//! CWE-1333: Simple character class pattern with no nesting or alternation.

use regex::Regex;

// vuln-code-snippet start testcodeRedos022
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let text = req.param("text");
    let re = Regex::new(r"^[a-z]+$").unwrap(); // vuln-code-snippet target-line testcodeRedos022
    let found = re.is_match(&text);
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos022
