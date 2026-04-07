use std::fs;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let log_name = req.param("log");
    let path = format!("/var/log/app/{}", log_name);
    match fs::read_to_string(&path) {
        Ok(content) => {
            let tail: Vec<&str> = content.lines().rev().take(100).collect();
            super::shared::BenchmarkResponse::ok(&tail.join("\n"))
        }
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
