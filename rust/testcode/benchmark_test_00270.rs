pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user = req.param("username");
    let session = req.param("session");
    let log_line = format!("Auth: user={} session={}", user, session);
    append_to_log(&log_line);
    super::shared::BenchmarkResponse::ok("OK")
}

fn append_to_log(line: &str) {
    eprintln!("[AUTH] {}", line);
}
