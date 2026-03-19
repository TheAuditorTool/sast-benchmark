//! ReDoS True Negative — CWE-1333
//! Hardcoded regex pattern only. User input is the haystack, never the pattern.

use regex::Regex;

const EMAIL_PATTERN: &str = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"; // vuln-code-snippet safe-line testcodeRedos004Safe

// vuln-code-snippet start testcodeRedos004Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("email");

    // SAFE: Pattern is a compile-time constant — user only controls the haystack
    let re = Regex::new(EMAIL_PATTERN).unwrap();
    let valid = re.is_match(&input);

    super::shared::BenchmarkResponse::ok(&format!("Valid email: {}", valid))
}
// vuln-code-snippet end testcodeRedos004Safe
