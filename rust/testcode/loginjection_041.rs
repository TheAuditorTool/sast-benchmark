//! CWE-117: User data sanitized on struct construction; safe representation used in log.

// vuln-code-snippet start testcodeLoginjection041
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let entry = SafeLogEntry::new(req.param("user"), req.param("action"));
    log_info(&entry.to_log_string()); // vuln-code-snippet target-line testcodeLoginjection041
    super::shared::BenchmarkResponse::ok("Done")
}

struct SafeLogEntry { user: String, action: String }

impl SafeLogEntry {
    fn new(user: String, action: String) -> Self {
        SafeLogEntry {
            user: user.replace('\n', "\\n").replace('\r', "\\r"),
            action: action.replace('\n', "\\n").replace('\r', "\\r"),
        }
    }
    fn to_log_string(&self) -> String {
        format!("user={} action={}", self.user, self.action)
    }
}

fn log_info(msg: &str) {
    eprintln!("[INFO] {}", msg);
}
// vuln-code-snippet end testcodeLoginjection041
