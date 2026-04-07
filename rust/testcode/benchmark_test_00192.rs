fn db_get_doc(id: &str) -> String {
    format!("document_data_for_{}", id)
}

fn get_session_user_id() -> String {
    "user_123".to_string()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    let user_id = get_session_user_id();
    if user_id != "" {
        let doc = db_get_doc(&id);
        return super::shared::BenchmarkResponse::ok(&doc);
    }
    super::shared::BenchmarkResponse::forbidden("not authenticated")
}
