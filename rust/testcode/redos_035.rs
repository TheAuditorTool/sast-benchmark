//! CWE-1333: Email-like pattern ^(([a-z])+.)+[A-Z]([a-z])+$ causes catastrophic backtracking on user-controlled text.

use fancy_regex::Regex;

// vuln-code-snippet start testcodeRedos035
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_text = req.param("text");

    let re = Regex::new(r"^(([a-z])+.)+[A-Z]([a-z])+$").unwrap();

    let found = re.is_match(&user_text).unwrap_or(false); // vuln-code-snippet target-line testcodeRedos035
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos035
