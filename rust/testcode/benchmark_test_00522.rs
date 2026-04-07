use std::collections::HashMap;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut paths = HashMap::new();
    paths.insert("user_path", req.param("path"));
    paths.insert("static_path", "config/safe.toml".to_string());
    let path = paths.get("static_path").unwrap();
    let content = std::fs::read_to_string(path.as_str());
    match content {
        Ok(data) => super::shared::BenchmarkResponse::ok(&data),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
