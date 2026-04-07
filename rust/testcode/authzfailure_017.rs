//! CWE-285: Checks user is non-empty but not that they own the document

fn db_get_doc(id: &str) -> String {
    format!("document_data_for_{}", id)
}

fn get_session_user_id() -> String {
    "user_123".to_string()
}

// vuln-code-snippet start testcodeAuthzfailure017
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    let user_id = get_session_user_id();
    if user_id != "" {
        let doc = db_get_doc(&id); // vuln-code-snippet target-line testcodeAuthzfailure017
        return super::shared::BenchmarkResponse::ok(&doc);
    }
    super::shared::BenchmarkResponse::forbidden("not authenticated")
}
// vuln-code-snippet end testcodeAuthzfailure017
