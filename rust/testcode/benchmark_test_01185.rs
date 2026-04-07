use std::fs;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let template = req.param("template");
    let content = req.body_str();
    let path = format!("/srv/templates/{}.html", template);
    match fs::write(&path, content.as_bytes()) {
        Ok(_) => super::shared::BenchmarkResponse::ok("Saved"),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
