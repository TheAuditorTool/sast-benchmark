//! ReDoS True Negative — CWE-1333
//! User selects from predefined pattern names, never provides raw regex.

use regex::Regex;

// vuln-code-snippet start testcodeRedos008Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let pattern_name = req.param("pattern_name");
    let text = req.param("text");

    // SAFE: Allowlist of known-safe patterns — user cannot inject arbitrary regex
    let pattern = match pattern_name.as_str() { // vuln-code-snippet safe-line testcodeRedos008Safe
        "digits" => r"^\d+$",
        "alpha" => r"^[a-zA-Z]+$",
        "email" => r"^[^@]+@[^@]+\.[^@]+$",
        _ => return super::shared::BenchmarkResponse::bad_request("Unknown pattern name"),
    };

    let re = Regex::new(pattern).unwrap();
    let found = re.is_match(&text);

    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos008Safe
