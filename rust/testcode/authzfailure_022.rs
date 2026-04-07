//! CWE-285: Horizontal privilege escalation — order detail accessible to anyone with order ID

fn db_get_order(order_id: &str) -> String {
    format!("order_details_for_{}", order_id)
}

// vuln-code-snippet start testcodeAuthzfailure022
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let order_id = req.param("order_id");
    let order = db_get_order(&order_id); // vuln-code-snippet target-line testcodeAuthzfailure022
    super::shared::BenchmarkResponse::ok(&order)
}
// vuln-code-snippet end testcodeAuthzfailure022
