//! CWE-1333: Dead-code branch — constant condition always selects safe hardcoded pattern; user_pattern never reaches Regex::new.

use regex::Regex;

// vuln-code-snippet start testcodeRedos039
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_pattern = req.param("pattern");
    let text = req.param("text");

    let re_src = if 7 * 6 > 40 { r"^[a-z]+$" } else { &user_pattern };

    let re = match Regex::new(re_src) { // vuln-code-snippet target-line testcodeRedos039
        Ok(r) => r,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e.to_string()),
    };

    let found = re.is_match(&text);
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos039
