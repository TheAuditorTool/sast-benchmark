//! CWE-20: Index from user input used for array access without non-negative validation.

// vuln-code-snippet start testcodeInputval036
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let index: i64 = req.param("index").parse().unwrap_or(0);
    let result = get_item_at(index); // vuln-code-snippet target-line testcodeInputval036
    super::shared::BenchmarkResponse::ok(&format!("item={}", result))
}

fn get_item_at(idx: i64) -> String { format!("item[{}]", idx) }
// vuln-code-snippet end testcodeInputval036
