//! Information Disclosure True Negative — CWE-209
//! Returns request reference ID for error tracking, no internal details.

// vuln-code-snippet start testcodeInfodisclosure009Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("data");

    let result: Result<i64, _> = input.parse();

    match result {
        Ok(val) => super::shared::BenchmarkResponse::ok(&format!("Parsed: {}", val)),
        // SAFE: Reference ID only, no internal error details
        Err(_) => super::shared::BenchmarkResponse::error( // vuln-code-snippet safe-line testcodeInfodisclosure009Safe
            "Error occurred, reference: REQ-12345",
        ),
    }
}
// vuln-code-snippet end testcodeInfodisclosure009Safe
