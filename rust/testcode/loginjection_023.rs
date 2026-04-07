//! CWE-117: User-Agent header written to stdout log without newline sanitization.

// vuln-code-snippet start testcodeLoginjection023
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let agent = req.header("User-Agent");
    println!("[ACCESS] agent={}", agent); // vuln-code-snippet target-line testcodeLoginjection023
    super::shared::BenchmarkResponse::ok("OK")
}
// vuln-code-snippet end testcodeLoginjection023
