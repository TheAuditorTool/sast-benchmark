use std::io::Write;
use std::process::{Command, Stdio};

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_data = req.body_str();

    let mut child = match Command::new("wc")
        .arg("-l")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn() {
        Ok(c) => c,
        Err(e) => return super::shared::BenchmarkResponse::error(&e.to_string()),
    };

    if let Some(ref mut stdin) = child.stdin {
        let _ = stdin.write_all(user_data.as_bytes());
    }

    match child.wait_with_output() {
        Ok(o) => super::shared::BenchmarkResponse::ok(
            &String::from_utf8_lossy(&o.stdout).to_string()
        ),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
