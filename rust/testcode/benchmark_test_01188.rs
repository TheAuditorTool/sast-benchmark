use std::fs;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.header("X-Filename");
    let base_dir = "/var/www/static/";
    let path = format!("{}{}", base_dir, filename);
    match fs::metadata(&path) {
        Ok(m) => super::shared::BenchmarkResponse::ok(&format!("Size: {}", m.len())),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
