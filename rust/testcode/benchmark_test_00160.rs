pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let entry = LogEntry {
        user: req.param("user"),
        action: req.param("action"),
    };
    write_log(&format!("user={} action={}", entry.user, entry.action));
    super::shared::BenchmarkResponse::ok("Done")
}

struct LogEntry { user: String, action: String }

fn write_log(msg: &str) {
    eprintln!("[LOG] {}", msg);
}
