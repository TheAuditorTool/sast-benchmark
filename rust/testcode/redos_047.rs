//! CWE-1333: regex crate with simple word-boundary pattern \bword\b on user text — linear-time NFA, immune to ReDoS.

use regex::Regex;

// vuln-code-snippet start testcodeRedos047
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let text = req.param("text");

    let re = Regex::new(r"\bword\b").unwrap();

    let found = re.is_match(&text); // vuln-code-snippet target-line testcodeRedos047
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos047
