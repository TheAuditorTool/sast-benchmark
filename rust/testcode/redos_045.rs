//! CWE-1333: Helper function ignores user input and always returns a safe hardcoded pattern.

use regex::Regex;

fn safe_pattern(_user: &str) -> &'static str {
    r"^[a-z]+$"
}

// vuln-code-snippet start testcodeRedos045
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_input = req.param("pattern");
    let text = req.param("text");

    let re = match Regex::new(safe_pattern(&user_input)) { // vuln-code-snippet target-line testcodeRedos045
        Ok(r) => r,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e.to_string()),
    };

    let found = re.is_match(&text);
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos045
