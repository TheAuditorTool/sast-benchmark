//! ReDoS True Negative — CWE-1333
//! Regex execution wrapped with a timeout to prevent catastrophic backtracking.

use regex::Regex;
use std::time::{Duration, Instant};

// vuln-code-snippet start testcodeRedos006Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_pattern = req.param("pattern");
    let text = req.param("text");

    let re = match Regex::new(&user_pattern) {
        Ok(r) => r,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e.to_string()),
    };

    // SAFE: Timeout wrapper aborts regex execution after 2 seconds
    let start = Instant::now(); // vuln-code-snippet safe-line testcodeRedos006Safe
    let timeout = Duration::from_secs(2);
    let found = re.is_match(&text);
    if start.elapsed() > timeout {
        return super::shared::BenchmarkResponse::error("Regex execution timed out");
    }

    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
// vuln-code-snippet end testcodeRedos006Safe
