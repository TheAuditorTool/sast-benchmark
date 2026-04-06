//! CWE-1333: Hardcoded anchored pattern with bounded quantifier, no backtracking risk.

use regex::Regex;

// vuln-code-snippet start testcodeRedos017
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let text = req.param("username");
    let re = Regex::new(r"^[a-zA-Z0-9]{1,64}$").unwrap(); // vuln-code-snippet target-line testcodeRedos017
    let valid = re.is_match(&text);
    super::shared::BenchmarkResponse::ok(&format!("Valid username: {}", valid))
}
// vuln-code-snippet end testcodeRedos017
