//! Path Traversal True Negative — CWE-22
//! Database lookup for file ID — no user-controlled filesystem path.

use std::collections::HashMap;

// vuln-code-snippet start testcodePathtraver012Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let file_id = req.param("id");

    // SAFE: Map ID to path via database lookup, user never controls path
    let mut file_map: HashMap<&str, &str> = HashMap::new(); // vuln-code-snippet safe-line testcodePathtraver012Safe
    file_map.insert("1", "/var/data/report.txt");
    file_map.insert("2", "/var/data/summary.csv");

    let path = match file_map.get(file_id.as_str()) {
        Some(p) => *p,
        None => return super::shared::BenchmarkResponse::bad_request("File not found"),
    };

    match std::fs::read_to_string(path) {
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver012Safe
