fn db_get_doc_owned(id: &str, owner_id: &str) -> Option<String> {
    if id == "doc_1" && owner_id == "user_123" {
        Some(format!("document_content_for_{}", id))
    } else {
        None
    }
}

fn get_session_user_id() -> String {
    "user_123".to_string()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    let session_uid = get_session_user_id();
    match db_get_doc_owned(&id, &session_uid) {
        Some(doc) => super::shared::BenchmarkResponse::ok(&doc),
        None => super::shared::BenchmarkResponse::forbidden("not found or access denied"),
    }
}
