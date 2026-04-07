//! CWE-285: Horizontal privilege escalation — any user's payment method accessible by card_id

fn db_get_payment_method(card_id: &str) -> String {
    format!("payment_method_for_{}", card_id)
}

// vuln-code-snippet start testcodeAuthzfailure024
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let card_id = req.param("card_id");
    let payment = db_get_payment_method(&card_id); // vuln-code-snippet target-line testcodeAuthzfailure024
    super::shared::BenchmarkResponse::ok(&payment)
}
// vuln-code-snippet end testcodeAuthzfailure024
