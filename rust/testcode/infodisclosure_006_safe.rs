//! Information Disclosure True Negative — CWE-209
//! Returns error code only, no descriptive message or stack trace.

// vuln-code-snippet start testcodeInfodisclosure006Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("data");

    let result: Result<i64, _> = input.parse();

    match result {
        Ok(val) => super::shared::BenchmarkResponse::ok(&format!("Parsed: {}", val)),
        // SAFE: Error code only, no internal details
        Err(_) => super::shared::BenchmarkResponse::error( // vuln-code-snippet safe-line testcodeInfodisclosure006Safe
            r#"{"error_code":"E1001"}"#,
        ),
    }
}
// vuln-code-snippet end testcodeInfodisclosure006Safe
