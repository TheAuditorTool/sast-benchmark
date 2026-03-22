//! CWE-1333: User selects from predefined pattern names via allowlist.

use regex::Regex;

// vuln-code-snippet start testcodeRedos008
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let pattern_name = req.param("pattern_name");
    let text = req.param("text");


    let pattern = match pattern_name.as_str() { // vuln-code-snippet target-line testcodeRedos008
        "digits" => r"^\d+$",
        "alpha" => r"^[a-zA-Z]+$",
        "email" => r"^[^@]+@[^@]+\.[^@]+$",
        _ => return super::shared::BenchmarkResponse::bad_request("Unknown pattern name"),
    };

    let re = Regex::new(pattern).unwrap();
    let found = re.is_match(&text);

    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos008
