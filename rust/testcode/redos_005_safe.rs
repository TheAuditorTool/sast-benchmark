//! ReDoS True Negative — CWE-1333
//! Pattern length limit rejects overly complex user-supplied regexes.

use regex::Regex;

// vuln-code-snippet start testcodeRedos005Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_pattern = req.param("pattern");
    let text = req.param("text");

    // SAFE: Reject patterns longer than 100 chars to limit complexity
    if user_pattern.len() > 100 { // vuln-code-snippet safe-line testcodeRedos005Safe
        return super::shared::BenchmarkResponse::bad_request("Pattern too long (max 100 chars)");
    }

    let re = match Regex::new(&user_pattern) {
        Ok(r) => r,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e.to_string()),
    };

    let found = re.is_match(&text);
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos005Safe
