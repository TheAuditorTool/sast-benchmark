//! ReDoS True Positive — CWE-1333
//! Timeout check runs AFTER regex execution, not during.
//! If is_match() hangs from catastrophic backtracking, the timeout never fires.
//! This is a common misconception — post-execution timeout does not prevent ReDoS.

use std::time::{Duration, Instant};

// vuln-code-snippet start testcodeRedos006Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_pattern = req.param("pattern");
    let text = req.param("text");

    let re = match regex::Regex::new(&user_pattern) {
        Ok(r) => r,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e.to_string()),
    };

    // VULNERABLE: Timeout check occurs AFTER is_match() returns, not during
    let start = Instant::now();
    let found = re.is_match(&text); // vuln-code-snippet vuln-line testcodeRedos006Safe
    if start.elapsed() > Duration::from_secs(2) {
        return super::shared::BenchmarkResponse::error("Regex execution timed out");
    }

    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos006Safe
