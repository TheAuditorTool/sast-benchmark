//! CWE-1333: Hardcoded regex pattern. User input is the haystack only.

use regex::Regex;

// vuln-code-snippet start testcodeRedos004
const EMAIL_PATTERN: &str = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"; // vuln-code-snippet target-line testcodeRedos004
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("email");


    let re = Regex::new(EMAIL_PATTERN).unwrap();
    let valid = re.is_match(&input);

    super::shared::BenchmarkResponse::ok(&format!("Valid email: {}", valid))
}
// vuln-code-snippet end testcodeRedos004
