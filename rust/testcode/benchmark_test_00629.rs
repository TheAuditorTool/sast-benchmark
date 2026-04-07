fn get_private_message(id: &str) -> String {
    format!("private_message_content_for_{}", id)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("message_id");
    let msg = get_private_message(&id);
    super::shared::BenchmarkResponse::ok(&msg)
}
