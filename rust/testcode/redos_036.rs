//! CWE-1333: regex crate with simple anchored pattern ^[a-z]+$ — linear-time NFA, immune to ReDoS.

use regex::Regex;

// vuln-code-snippet start testcodeRedos036
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let text = req.param("text");

    let re = Regex::new(r"^[a-z]+$").unwrap();

    let found = re.is_match(&text); // vuln-code-snippet target-line testcodeRedos036
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos036
