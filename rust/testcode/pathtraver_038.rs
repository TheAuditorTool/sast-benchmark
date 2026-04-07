//! CWE-22: HashMap lookup uses "safe" key; tainted value stored under "user" key is never read.

use std::collections::HashMap;

// vuln-code-snippet start testcodePathtraver038
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut m: HashMap<&str, String> = HashMap::new();
    m.insert("user", req.param("f"));
    m.insert("safe", "/files/report.txt".to_string());

    let p = m.get("safe").unwrap();
    match std::fs::read_to_string(p) { // vuln-code-snippet target-line testcodePathtraver038
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver038
