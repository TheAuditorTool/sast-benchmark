//! CWE-532: Only non-sensitive request ID logged; credentials and tokens not included.

// vuln-code-snippet start testcodeInfodisclosure037
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let request_id = req.param("request_id");
    log_info(&format!("Processing request_id={}", request_id)); // vuln-code-snippet target-line testcodeInfodisclosure037
    super::shared::BenchmarkResponse::ok("OK")
}

fn log_info(msg: &str) { eprintln!("[INFO] {}", msg); }
// vuln-code-snippet end testcodeInfodisclosure037
