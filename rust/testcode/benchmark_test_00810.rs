fn db_get_any_document(id: &str) -> String {
    format!("any_document_for_{}", id)
}

fn db_get_owned_document(id: &str, session_uid: &str) -> String {
    format!("owned_document_for_{}_{}", id, session_uid)
}

fn get_session_uid() -> String {
    "user_123".to_string()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    let session_uid = get_session_uid();
    let result = if 1 > 2 {
        db_get_any_document(&id)
    } else {
        db_get_owned_document(&id, &session_uid)
    };
    super::shared::BenchmarkResponse::ok(&result)
}
