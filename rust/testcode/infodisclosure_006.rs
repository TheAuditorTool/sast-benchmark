//! CWE-209: Returns error code only, no descriptive message.

// vuln-code-snippet start testcodeInfodisclosure006
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("data");

    let result: Result<i64, _> = input.parse();

    match result {
        Ok(val) => super::shared::BenchmarkResponse::ok(&format!("Parsed: {}", val)),
        Err(_) => super::shared::BenchmarkResponse::error( // vuln-code-snippet target-line testcodeInfodisclosure006
            r#"{"error_code":"E1001"}"#,
        ),
    }
}
// vuln-code-snippet end testcodeInfodisclosure006
