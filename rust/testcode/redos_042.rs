//! CWE-1333: Overwrite neutralises taint — user pattern is immediately overwritten with a hardcoded safe literal before compile.

use regex::Regex;

// vuln-code-snippet start testcodeRedos042
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let text = req.param("text");

    let mut pattern = req.param("pat");
    pattern = r"^[a-z]+$".to_string();

    let re = match Regex::new(&pattern) { // vuln-code-snippet target-line testcodeRedos042
        Ok(r) => r,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e.to_string()),
    };

    let found = re.is_match(&text);
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos042
