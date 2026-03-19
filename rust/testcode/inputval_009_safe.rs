//! Input Validation True Negative — CWE-20
//! String length limit enforced before processing.

// vuln-code-snippet start testcodeInputval009Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("data");

    // SAFE: Length limit prevents oversized input from consuming resources
    if input.len() > 1024 { // vuln-code-snippet safe-line testcodeInputval009Safe
        return super::shared::BenchmarkResponse::bad_request("Input exceeds 1024 character limit");
    }

    let processed = input.to_uppercase();
    super::shared::BenchmarkResponse::ok(&format!("Processed: {}", processed))
}
// vuln-code-snippet end testcodeInputval009Safe
