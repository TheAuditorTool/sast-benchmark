//! CWE-1333: regex crate with case-insensitive word-boundary pattern — no catastrophic backtracking possible.

use regex::Regex;

// vuln-code-snippet start testcodeRedos046
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let text = req.param("text");

    let re = Regex::new(r"(?i)hello\b").unwrap();

    let found = re.is_match(&text); // vuln-code-snippet target-line testcodeRedos046
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos046
