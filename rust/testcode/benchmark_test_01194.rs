use std::fs;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let plugin = req.cookie("plugin");
    let path = format!("/plugins/{}/manifest.json", plugin);
    match fs::read_to_string(&path) {
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
