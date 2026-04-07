fn db_get_note(note_id: &str) -> String {
    format!("private_note_content_for_{}", note_id)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let note_id = req.param("note_id");
    let note = db_get_note(&note_id);
    super::shared::BenchmarkResponse::ok(&note)
}
