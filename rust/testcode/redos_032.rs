//! CWE-1333: User prefix interpolated into catastrophic pattern via format!, backtracking engine runs on user text.

use fancy_regex::Regex;

// vuln-code-snippet start testcodeRedos032
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let prefix = req.param("prefix");
    let text = req.param("text");

    let pat = format!("({}+)+$", prefix);

    let re = match Regex::new(&pat) { // vuln-code-snippet target-line testcodeRedos032
        Ok(r) => r,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e.to_string()),
    };

    let found = re.is_match(&text).unwrap_or(false);
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos032
