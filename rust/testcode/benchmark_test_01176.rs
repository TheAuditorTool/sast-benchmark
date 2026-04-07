use std::process::{Command, Stdio};

struct CompileJob {
    source: String,
    output: String,
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let job = CompileJob {
        source: req.param("src"),
        output: req.param("out"),
    };
    let cmd = format!("gcc {} -o {}", job.source, job.output);
    let output = Command::new("sh").arg("-c").arg(&cmd)
        .stdout(Stdio::piped()).stderr(Stdio::piped()).output();
    match output {
        Ok(o) => super::shared::BenchmarkResponse::ok(&String::from_utf8_lossy(&o.stdout).to_string()),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
