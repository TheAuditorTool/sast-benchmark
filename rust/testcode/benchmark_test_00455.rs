pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let referer = req.header("referer");

    file_log(&format!("[AUDIT] referer={}", referer));

    super::shared::BenchmarkResponse::ok("Logged")
}

fn file_log(msg: &str) {
    use std::io::Write;
    if let Ok(mut f) = std::fs::OpenOptions::new().append(true).open("/var/log/app.log") {
        let _ = writeln!(f, "{}", msg);
    }
}
