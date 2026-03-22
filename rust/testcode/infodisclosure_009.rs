//! CWE-209: Returns request reference ID for error tracking.

// vuln-code-snippet start testcodeInfodisclosure009
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("data");

    let result: Result<i64, _> = input.parse();

    match result {
        Ok(val) => super::shared::BenchmarkResponse::ok(&format!("Parsed: {}", val)),
        Err(_) => super::shared::BenchmarkResponse::error( // vuln-code-snippet target-line testcodeInfodisclosure009
            "Error occurred, reference: REQ-12345",
        ),
    }
}
// vuln-code-snippet end testcodeInfodisclosure009
