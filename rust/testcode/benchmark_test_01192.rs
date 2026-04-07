use std::fs;
use std::path::Path;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let prefix = req.param("prefix");
    let name = req.param("name");
    let combined = format!("{}/{}", prefix, name);
    let p = Path::new(&combined);
    match fs::read_to_string(p) {
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
