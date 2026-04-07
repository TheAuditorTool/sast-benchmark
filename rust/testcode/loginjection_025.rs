//! CWE-117: User-supplied resource identifier embedded in error log message.

// vuln-code-snippet start testcodeLoginjection025
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let resource = req.param("resource");
    let msg = format!("Resource not found: {}", resource);
    log_error(&msg); // vuln-code-snippet target-line testcodeLoginjection025
    super::shared::BenchmarkResponse::bad_request("Not found")
}

fn log_error(msg: &str) {
    eprintln!("[ERROR] {}", msg);
}
// vuln-code-snippet end testcodeLoginjection025
