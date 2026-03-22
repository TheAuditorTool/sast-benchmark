//! CWE-209: Detailed errors gated by debug_mode flag (hardcoded false).

// vuln-code-snippet start testcodeInfodisclosure010
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let debug_mode = false;
    let input = req.param("data");

    let result: Result<i64, _> = input.parse();

    match result {
        Ok(val) => super::shared::BenchmarkResponse::ok(&format!("Parsed: {}", val)),
        Err(e) => {
            if debug_mode { // vuln-code-snippet target-line testcodeInfodisclosure010
                super::shared::BenchmarkResponse::error(&format!("Debug: {:?}", e))
            } else {
                super::shared::BenchmarkResponse::error("Internal Server Error")
            }
        }
    }
}
// vuln-code-snippet end testcodeInfodisclosure010
