//! Information Disclosure True Positive — CWE-209
//! Stack trace leaked in HTTP error response via debug format {:?}.

// vuln-code-snippet start testcodeInfodisclosure001Vulnerable
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("data");

    let result: Result<i64, _> = input.parse();

    match result {
        Ok(val) => super::shared::BenchmarkResponse::ok(&format!("Parsed: {}", val)),
        // VULNERABLE: Full error details leaked to client via {:?}
        Err(e) => super::shared::BenchmarkResponse::error( // vuln-code-snippet vuln-line testcodeInfodisclosure001Vulnerable
            &format!("Error processing request: {:?}\nStack context: input={}, handler=parse_data", e, input),
        ),
    }
}
// vuln-code-snippet end testcodeInfodisclosure001Vulnerable
