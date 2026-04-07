//! CWE-1333: Hardcoded lookahead pattern (?=(a+))\1+ requires fancy_regex; user-controlled text triggers catastrophic backtracking.

use fancy_regex::Regex;

// vuln-code-snippet start testcodeRedos034
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_text = req.param("text");

    let re = Regex::new(r"(?=(a+))\1+").unwrap();

    let found = re.is_match(&user_text).unwrap_or(false); // vuln-code-snippet target-line testcodeRedos034
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos034
