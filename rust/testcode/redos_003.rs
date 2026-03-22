//! CWE-1333: User-provided search pattern applied to large text.

use fancy_regex::Regex;

// vuln-code-snippet start testcodeRedos003
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let search_pattern = req.param("search");
    let large_text = "a]".repeat(100_000);

    let re = match Regex::new(&search_pattern) {
        Ok(r) => r,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e.to_string()),
    };

    let found = re.is_match(&large_text).unwrap_or(false); // vuln-code-snippet target-line testcodeRedos003

    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos003
