//! Information Disclosure True Negative — CWE-209
//! Same error scenario as #1 but returns generic message without internal details.

// vuln-code-snippet start testcodeInfodisclosure004Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("data");

    let result: Result<i64, _> = input.parse();

    match result {
        Ok(val) => super::shared::BenchmarkResponse::ok(&format!("Parsed: {}", val)),
        // SAFE: Generic error message, no internal details
        Err(_) => super::shared::BenchmarkResponse::error( // vuln-code-snippet safe-line testcodeInfodisclosure004Safe
            "Internal Server Error",
        ),
    }
}
// vuln-code-snippet end testcodeInfodisclosure004Safe
