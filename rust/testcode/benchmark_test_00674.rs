fn download_file(path: &str) -> Vec<u8> {
    format!("file_bytes_for_{}", path).into_bytes()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let file_id = req.param("file_id");
    let bytes = download_file(&file_id);
    super::shared::BenchmarkResponse::ok(&String::from_utf8_lossy(&bytes))
}
