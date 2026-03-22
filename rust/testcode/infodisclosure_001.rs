//! CWE-209: Error details in HTTP response via debug format {:?}.

// vuln-code-snippet start testcodeInfodisclosure001
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("data");

    let result: Result<i64, _> = input.parse();

    match result {
        Ok(val) => super::shared::BenchmarkResponse::ok(&format!("Parsed: {}", val)),
        Err(e) => super::shared::BenchmarkResponse::error( // vuln-code-snippet target-line testcodeInfodisclosure001
            &format!("Error processing request: {:?}\nStack context: input={}, handler=parse_data", e, input),
        ),
    }
}
// vuln-code-snippet end testcodeInfodisclosure001
