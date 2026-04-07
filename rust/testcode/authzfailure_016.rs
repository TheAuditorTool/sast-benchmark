//! CWE-285: Ownership check present only as TODO comment, never enforced

fn db_get_document(id: &str) -> String {
    format!("document_content_for_{}", id)
}

// vuln-code-snippet start testcodeAuthzfailure016
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    // TODO: check ownership before returning document
    let doc = db_get_document(&id); // vuln-code-snippet target-line testcodeAuthzfailure016
    super::shared::BenchmarkResponse::ok(&doc)
}
// vuln-code-snippet end testcodeAuthzfailure016
