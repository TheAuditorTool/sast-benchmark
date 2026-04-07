//! CWE-362: Helper function discards user input and returns a safe constant operation result.

// vuln-code-snippet start testcodeRaceCondition048
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let path = req.param("path");
    let content = safe_read(&path); // vuln-code-snippet target-line testcodeRaceCondition048
    super::shared::BenchmarkResponse::ok(&content)
}

fn safe_read(_user_path: &str) -> String {
    "static-content".to_string()
}
// vuln-code-snippet end testcodeRaceCondition048
