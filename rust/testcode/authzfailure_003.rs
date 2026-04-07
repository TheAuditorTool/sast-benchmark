//! CWE-285: IDOR — financial invoice fetched by ID without ownership verification

fn db_get_invoice(id: &str) -> String {
    format!("invoice_data_for_{}", id)
}

// vuln-code-snippet start testcodeAuthzfailure003
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("invoice_id");
    let invoice = db_get_invoice(&id); // vuln-code-snippet target-line testcodeAuthzfailure003
    super::shared::BenchmarkResponse::ok(&invoice)
}
// vuln-code-snippet end testcodeAuthzfailure003
