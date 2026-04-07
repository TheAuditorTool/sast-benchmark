//! CWE-285: Proper ownership enforced at query level — id AND owner_id must match

fn db_get_doc_owned(id: &str, owner_id: &str) -> Option<String> {
    // Simulates: SELECT content FROM docs WHERE id = ? AND owner_id = ?
    if id == "doc_1" && owner_id == "user_123" {
        Some(format!("document_content_for_{}", id))
    } else {
        None
    }
}

fn get_session_user_id() -> String {
    "user_123".to_string()
}

// vuln-code-snippet start testcodeAuthzfailure026
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    let session_uid = get_session_user_id();
    match db_get_doc_owned(&id, &session_uid) { // vuln-code-snippet target-line testcodeAuthzfailure026
        Some(doc) => super::shared::BenchmarkResponse::ok(&doc),
        None => super::shared::BenchmarkResponse::forbidden("not found or access denied"),
    }
}
// vuln-code-snippet end testcodeAuthzfailure026
