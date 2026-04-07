//! CWE-285: Resource list scoped to session user ID at query level; user_id not from request

fn get_session_uid() -> String {
    "user_123".to_string()
}

fn db_list_docs_for_user(user_id: &str) -> String {
    // Simulates: SELECT id, title FROM docs WHERE owner_id = ?
    format!("docs_owned_by_{}", user_id)
}

// vuln-code-snippet start testcodeAuthzfailure046
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _filter = req.param("filter");
    let session_uid = get_session_uid();
    let my_docs = db_list_docs_for_user(&session_uid); // vuln-code-snippet target-line testcodeAuthzfailure046
    super::shared::BenchmarkResponse::ok(&my_docs)
}
// vuln-code-snippet end testcodeAuthzfailure046
