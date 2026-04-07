//! CWE-1333: regex crate with bounded digit pattern ^\d{1,10}$ — linear-time NFA, immune to ReDoS.

use regex::Regex;

// vuln-code-snippet start testcodeRedos037
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let text = req.param("text");

    let re = Regex::new(r"^\d{1,10}$").unwrap();

    let found = re.is_match(&text); // vuln-code-snippet target-line testcodeRedos037
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos037
