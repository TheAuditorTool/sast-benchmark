//! CWE-1333: Input length capped before regex evaluation.

use fancy_regex::Regex;

// vuln-code-snippet start testcodeRedos020
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let text = req.param("text");
    if text.len() > 256 { // vuln-code-snippet target-line testcodeRedos020
        return super::shared::BenchmarkResponse::bad_request("Input too long");
    }
    let re = Regex::new(r"(a+)+$").unwrap();
    let found = re.is_match(&text).unwrap_or(false);
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos020
