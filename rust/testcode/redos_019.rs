//! CWE-1333: User input matched against fixed patterns via match statement, no regex.

// vuln-code-snippet start testcodeRedos019
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let action = req.param("action");
    let result = match action.as_str() { // vuln-code-snippet target-line testcodeRedos019
        "create" | "read" | "update" | "delete" => format!("Action: {}", action),
        _ => "Unknown action".to_string(),
    };
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeRedos019
