//! ReDoS True Positive — CWE-1333
//! Nested quantifier pattern built from user input creates exponential backtracking.

use regex::Regex;

// vuln-code-snippet start testcodeRedos002Vulnerable
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_input = req.param("keyword");
    let text = req.param("text");

    // VULNERABLE: Wrapping user input in nested quantifiers creates catastrophic backtracking
    let pattern = format!("({})+$", user_input); // vuln-code-snippet vuln-line testcodeRedos002Vulnerable

    let re = match Regex::new(&pattern) {
        Ok(r) => r,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e.to_string()),
    };

    let found = re.is_match(&text);
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos002Vulnerable
