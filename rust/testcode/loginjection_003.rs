//! CWE-117: User-controlled error message written to stderr.

// vuln-code-snippet start testcodeLoginjection003
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_msg = req.param("message");

    eprintln!("Error processing request: {}", user_msg); // vuln-code-snippet target-line testcodeLoginjection003

    super::shared::BenchmarkResponse::error("Processing failed")
}
// vuln-code-snippet end testcodeLoginjection003
