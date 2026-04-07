use std::path::Path;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");
    let pid_path = Path::new("/var/run/app.pid");

    if !pid_path.exists() {
        let pid = std::process::id();
        let _ = std::fs::write(pid_path, pid.to_string());
        super::shared::BenchmarkResponse::ok(&format!("Started with PID {}", pid))
    } else {
        super::shared::BenchmarkResponse::error("Already running")
    }
}
