//! CWE-1333: User-supplied regex compiled inside request handler on every call.

use fancy_regex::Regex;

// vuln-code-snippet start testcodeRedos014
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let pattern = req.param("pattern");
    let text = req.param("text");
    let re = match Regex::new(&pattern) { // vuln-code-snippet target-line testcodeRedos014
        Ok(r) => r,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e.to_string()),
    };
    let found = re.is_match(&text).unwrap_or(false);
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos014
