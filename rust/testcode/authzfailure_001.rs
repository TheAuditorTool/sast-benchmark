//! CWE-285: IDOR — document fetched by ID without session ownership check

fn db_get_document(id: &str) -> String {
    format!("document_content_for_{}", id)
}

// vuln-code-snippet start testcodeAuthzfailure001
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    let doc = db_get_document(&id); // vuln-code-snippet target-line testcodeAuthzfailure001
    super::shared::BenchmarkResponse::ok(&doc)
}
// vuln-code-snippet end testcodeAuthzfailure001
