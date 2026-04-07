use std::process::{Command, Stdio};
use std::time::Duration;

pub fn handle(_req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut child = match Command::new("sleep")
        .arg("10")
        .stdout(Stdio::piped())
        .spawn() {
        Ok(c) => c,
        Err(e) => return super::shared::BenchmarkResponse::error(&e.to_string()),
    };

    std::thread::sleep(Duration::from_secs(2));
    let _ = child.kill();

    super::shared::BenchmarkResponse::ok("Process timed out and killed")
}
