//! CWE-285: Fetch then explicit ownership check before returning document

struct Document {
    owner_id: String,
    content: String,
}

fn db_get_document(id: &str) -> Option<Document> {
    Some(Document {
        owner_id: "user_123".to_string(),
        content: format!("document_content_for_{}", id),
    })
}

fn get_session_user_id() -> String {
    "user_123".to_string()
}

// vuln-code-snippet start testcodeAuthzfailure029
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    let session_user_id = get_session_user_id();
    let doc = match db_get_document(&id) {
        Some(d) => d,
        None => return super::shared::BenchmarkResponse::error("not found"),
    };
    if doc.owner_id != session_user_id { // vuln-code-snippet target-line testcodeAuthzfailure029
        return super::shared::BenchmarkResponse::forbidden("access denied");
    }
    super::shared::BenchmarkResponse::ok(&doc.content)
}
// vuln-code-snippet end testcodeAuthzfailure029
