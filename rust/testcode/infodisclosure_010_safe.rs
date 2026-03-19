//! Information Disclosure True Negative — CWE-209
//! Only returns detailed errors if debug mode is enabled (hardcoded false).

// vuln-code-snippet start testcodeInfodisclosure010Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let debug_mode = false;
    let input = req.param("data");

    let result: Result<i64, _> = input.parse();

    match result {
        Ok(val) => super::shared::BenchmarkResponse::ok(&format!("Parsed: {}", val)),
        Err(e) => {
            // SAFE: Debug mode is off, detailed error is never sent
            if debug_mode { // vuln-code-snippet safe-line testcodeInfodisclosure010Safe
                super::shared::BenchmarkResponse::error(&format!("Debug: {:?}", e))
            } else {
                super::shared::BenchmarkResponse::error("Internal Server Error")
            }
        }
    }
}
// vuln-code-snippet end testcodeInfodisclosure010Safe
