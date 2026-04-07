//! CWE-117: Full request body written to debug log enabling log injection via body content.

// vuln-code-snippet start testcodeLoginjection035
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.param("body");
    log_debug(&format!("Request body: {}", body)); // vuln-code-snippet target-line testcodeLoginjection035
    super::shared::BenchmarkResponse::ok("Processed")
}

fn log_debug(msg: &str) {
    eprintln!("[DEBUG] {}", msg);
}
// vuln-code-snippet end testcodeLoginjection035
