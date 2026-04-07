use std::path::Path;
use std::process::{Command, Stdio};

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filepath = req.param("path");
    let path = Path::new(&filepath);

    if !path.exists() || !path.is_file() {
        return super::shared::BenchmarkResponse::bad_request("Invalid file path");
    }

    let canonical = match path.canonicalize() {
        Ok(p) => p,
        Err(_) => return super::shared::BenchmarkResponse::error("Invalid path"),
    };
    if !canonical.starts_with("/allowed/dir") {
        return super::shared::BenchmarkResponse::bad_request("Path outside allowed directory");
    }

    let output = Command::new("cat")
        .arg(canonical)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();

    match output {
        Ok(o) => super::shared::BenchmarkResponse::ok(
            &String::from_utf8_lossy(&o.stdout).to_string()
        ),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
