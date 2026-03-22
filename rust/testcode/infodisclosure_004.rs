//! CWE-209: Generic error message returned without internal details.

// vuln-code-snippet start testcodeInfodisclosure004
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("data");

    let result: Result<i64, _> = input.parse();

    match result {
        Ok(val) => super::shared::BenchmarkResponse::ok(&format!("Parsed: {}", val)),
        Err(_) => super::shared::BenchmarkResponse::error( // vuln-code-snippet target-line testcodeInfodisclosure004
            "Internal Server Error",
        ),
    }
}
// vuln-code-snippet end testcodeInfodisclosure004
