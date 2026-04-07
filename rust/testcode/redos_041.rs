//! CWE-1333: regex crate with safe email pattern on user text — linear-time NFA, immune to ReDoS.

use regex::Regex;

// vuln-code-snippet start testcodeRedos041
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let text = req.param("text");

    let re = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();

    let found = re.is_match(&text); // vuln-code-snippet target-line testcodeRedos041
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos041
