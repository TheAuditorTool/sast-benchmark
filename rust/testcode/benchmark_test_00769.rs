use std::collections::HashMap;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    let mut messages = HashMap::new();
    messages.insert("internal", format!("SQL error for id={}: column missing", id));
    messages.insert("safe", "Request could not be completed".to_string());
    let msg = messages.get("safe").unwrap();
    super::shared::BenchmarkResponse::error(msg)
}
