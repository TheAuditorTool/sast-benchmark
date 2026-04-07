use std::process::{Command, Stdio};

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let domain = req.param("domain");
    let qtype = req.param("type");
    let record_type = match qtype.as_str() {
        "A" | "AAAA" | "MX" | "TXT" | "NS" => qtype.as_str(),
        _ => "A",
    };
    let output = Command::new("dig")
        .arg(record_type)
        .arg(&domain)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();
    match output {
        Ok(o) => super::shared::BenchmarkResponse::ok(&String::from_utf8_lossy(&o.stdout).to_string()),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
