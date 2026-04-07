//! CWE-1333: regex crate with hardcoded safe pattern on user text; length guard limits input to < 20 chars.

use regex::Regex;

// vuln-code-snippet start testcodeRedos043
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let text = req.param("text");

    if text.len() >= 20 {
        return super::shared::BenchmarkResponse::bad_request("Input too long");
    }

    let re = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();

    let found = re.is_match(&text); // vuln-code-snippet target-line testcodeRedos043
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos043
