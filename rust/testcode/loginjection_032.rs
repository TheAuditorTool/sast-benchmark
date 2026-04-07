//! CWE-117: Username length validated but newlines not stripped, injection still possible.

// vuln-code-snippet start testcodeLoginjection032
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    if username.len() > 64 {
        return super::shared::BenchmarkResponse::bad_request("Too long");
    }
    audit_log(&format!("user={}", username)); // vuln-code-snippet target-line testcodeLoginjection032
    super::shared::BenchmarkResponse::ok("OK")
}

fn audit_log(msg: &str) {
    eprintln!("[AUDIT] {}", msg);
}
// vuln-code-snippet end testcodeLoginjection032
