fn db_update_post(post_id: &str, content: &str) -> String {
    format!("post_{}_updated_with_{}", post_id, content)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let post_id = req.param("post_id");
    let content = req.param("content");
    let result = db_update_post(&post_id, &content);
    super::shared::BenchmarkResponse::ok(&result)
}
