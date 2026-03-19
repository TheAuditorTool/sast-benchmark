//! ReDoS True Negative — CWE-1333
//! User input escaped to remove all regex metacharacters before matching.

use regex::Regex;

// vuln-code-snippet start testcodeRedos007Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_input = req.param("search");
    let text = req.param("text");

    // SAFE: Escape all regex metacharacters — user input becomes a literal string search
    let escaped = regex::escape(&user_input); // vuln-code-snippet safe-line testcodeRedos007Safe

    let re = Regex::new(&escaped).unwrap();
    let found = re.is_match(&text);

    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos007Safe
