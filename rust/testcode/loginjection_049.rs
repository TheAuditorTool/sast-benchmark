//! CWE-117: Branch always evaluates to sanitized path due to compile-time constant condition.

// vuln-code-snippet start testcodeLoginjection049
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user = req.param("user");
    if 2 + 2 == 4 {
        let safe = user.replace(['\n', '\r', '\t'], " ");
        log_info(&format!("user={}", safe)); // vuln-code-snippet target-line testcodeLoginjection049
    } else {
        log_info(&format!("user={}", user));
    }
    super::shared::BenchmarkResponse::ok("OK")
}

fn log_info(msg: &str) {
    eprintln!("[INFO] {}", msg);
}
// vuln-code-snippet end testcodeLoginjection049
