//! CWE-117: User input stored in LogEntry struct and written to log without sanitization.

// vuln-code-snippet start testcodeLoginjection026
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let entry = LogEntry {
        user: req.param("user"),
        action: req.param("action"),
    };
    write_log(&format!("user={} action={}", entry.user, entry.action)); // vuln-code-snippet target-line testcodeLoginjection026
    super::shared::BenchmarkResponse::ok("Done")
}

struct LogEntry { user: String, action: String }

fn write_log(msg: &str) {
    eprintln!("[LOG] {}", msg);
}
// vuln-code-snippet end testcodeLoginjection026
