fn db_get_document(id: &str) -> String {
    format!("document_content_for_{}", id)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    let doc = db_get_document(&id);
    super::shared::BenchmarkResponse::ok(&doc)
}
