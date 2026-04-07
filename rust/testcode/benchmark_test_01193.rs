use std::fs;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let archive = req.param("archive");
    let entry = req.param("entry");
    let path = format!("/data/archives/{}/{}", archive, entry);
    match fs::read(&path) {
        Ok(data) => super::shared::BenchmarkResponse::ok(&format!("Read {} bytes", data.len())),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
