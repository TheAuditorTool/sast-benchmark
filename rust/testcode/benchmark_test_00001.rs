fn get_session_uid() -> String {
    "user_123".to_string()
}

fn db_list_docs_for_user(user_id: &str) -> String {
    format!("docs_owned_by_{}", user_id)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _filter = req.param("filter");
    let session_uid = get_session_uid();
    let my_docs = db_list_docs_for_user(&session_uid);
    super::shared::BenchmarkResponse::ok(&my_docs)
}
