//! CWE-1333: regex crate with bounded alphanumeric pattern ^[a-zA-Z0-9_]{1,64}$ — safe by construction.

use regex::Regex;

// vuln-code-snippet start testcodeRedos048
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let text = req.param("text");

    let re = Regex::new(r"^[a-zA-Z0-9_]{1,64}$").unwrap();

    let found = re.is_match(&text); // vuln-code-snippet target-line testcodeRedos048
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos048
