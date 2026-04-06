//! CWE-1333: User-supplied pattern with backreference compiled via backtracking engine.

use fancy_regex::Regex;

// vuln-code-snippet start testcodeRedos010
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let text = req.param("text");
    let re = Regex::new(r"(.+)\1").unwrap(); // vuln-code-snippet target-line testcodeRedos010
    let found = re.is_match(&text).unwrap_or(false);
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos010
