//! CWE-362: PID file created after non-atomic existence check.

use std::path::Path;

// vuln-code-snippet start testcodeRaceCondition010
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");
    let pid_path = Path::new("/var/run/app.pid");

    if !pid_path.exists() { // vuln-code-snippet target-line testcodeRaceCondition010
        let pid = std::process::id();
        let _ = std::fs::write(pid_path, pid.to_string());
        super::shared::BenchmarkResponse::ok(&format!("Started with PID {}", pid))
    } else {
        super::shared::BenchmarkResponse::error("Already running")
    }
}
// vuln-code-snippet end testcodeRaceCondition010
