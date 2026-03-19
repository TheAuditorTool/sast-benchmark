//! ReDoS True Positive — CWE-1333
//! User-provided search pattern applied to large text without restrictions.

use regex::Regex;

// vuln-code-snippet start testcodeRedos003Vulnerable
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let search_pattern = req.param("search");
    let large_text = "a]".repeat(100_000);

    let re = match Regex::new(&search_pattern) {
        Ok(r) => r,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e.to_string()),
    };

    // VULNERABLE: User pattern matched against large text — DoS via slow regex
    let found = re.is_match(&large_text); // vuln-code-snippet vuln-line testcodeRedos003Vulnerable

    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos003Vulnerable
