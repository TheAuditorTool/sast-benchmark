//! CWE-1333: User input escaped via regex::escape before matching.

use regex::Regex;

// vuln-code-snippet start testcodeRedos007
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_input = req.param("search");
    let text = req.param("text");


    let escaped = regex::escape(&user_input); // vuln-code-snippet target-line testcodeRedos007

    let re = Regex::new(&escaped).unwrap();
    let found = re.is_match(&text);

    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos007
