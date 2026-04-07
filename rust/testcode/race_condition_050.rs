//! CWE-362: User-tainted path stored under one HashMap key; safe constant path read from another key.

use std::collections::HashMap;

// vuln-code-snippet start testcodeRaceCondition050
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut paths = HashMap::new();
    paths.insert("user_path", req.param("path"));
    paths.insert("static_path", "config/safe.toml".to_string());
    let path = paths.get("static_path").unwrap();
    let content = std::fs::read_to_string(path.as_str()); // vuln-code-snippet target-line testcodeRaceCondition050
    match content {
        Ok(data) => super::shared::BenchmarkResponse::ok(&data),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodeRaceCondition050
