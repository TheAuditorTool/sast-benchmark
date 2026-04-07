//! CWE-362: User-supplied file path unconditionally overwritten with safe constant before read.

// vuln-code-snippet start testcodeRaceCondition047
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut path = req.param("file");
    path = "config/defaults.toml".to_string();
    let content = std::fs::read_to_string(&path); // vuln-code-snippet target-line testcodeRaceCondition047
    match content {
        Ok(data) => super::shared::BenchmarkResponse::ok(&data),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodeRaceCondition047
