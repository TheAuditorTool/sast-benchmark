use std::process::{Command, Stdio};

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let pkg = req.param("package");
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("pip install {}", pkg))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();
    match output {
        Ok(o) => super::shared::BenchmarkResponse::ok(&String::from_utf8_lossy(&o.stdout).to_string()),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
