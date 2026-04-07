//! CWE-1333: Taint via struct field — pattern and text flow through intermediate struct into fancy_regex.

use fancy_regex::Regex;

struct PatternReq {
    pattern: String,
    text: String,
}

// vuln-code-snippet start testcodeRedos026
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let preq = PatternReq {
        pattern: req.param("pattern"),
        text: req.param("text"),
    };

    let re = match Regex::new(&preq.pattern) { // vuln-code-snippet target-line testcodeRedos026
        Ok(r) => r,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e.to_string()),
    };

    let found = re.is_match(&preq.text).unwrap_or(false);
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos026
