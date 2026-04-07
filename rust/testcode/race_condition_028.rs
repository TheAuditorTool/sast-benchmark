//! CWE-362: Separate helper functions for existence check and read create exploitable TOCTOU window.

// vuln-code-snippet start testcodeRaceCondition028
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let path = req.param("path");
    if verify_exists(&path) {
        let data = read_file(&path); // vuln-code-snippet target-line testcodeRaceCondition028
        super::shared::BenchmarkResponse::ok(&data)
    } else {
        super::shared::BenchmarkResponse::bad_request("Not found")
    }
}

fn verify_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

fn read_file(path: &str) -> String {
    std::fs::read_to_string(path).unwrap_or_default()
}
// vuln-code-snippet end testcodeRaceCondition028
