//! CWE-532: Entire request body including sensitive fields written to application log.

// vuln-code-snippet start testcodeInfodisclosure033
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.param("body");
    log_info(&format!("Request body: {}", body)); // vuln-code-snippet target-line testcodeInfodisclosure033
    super::shared::BenchmarkResponse::ok("Processed")
}

fn log_info(msg: &str) { eprintln!("[INFO] {}", msg); }
// vuln-code-snippet end testcodeInfodisclosure033
