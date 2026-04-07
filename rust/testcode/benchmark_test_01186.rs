use std::fs;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let dir = req.param("dir");
    let file = req.param("file");
    let full_path = format!("/uploads/{}/{}", dir, file);
    match fs::read(&full_path) {
        Ok(data) => super::shared::BenchmarkResponse::ok(&format!("Read {} bytes", data.len())),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
