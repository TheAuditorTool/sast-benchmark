pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content = req.param("content");
    save_upload(&filename, content.as_bytes());
    super::shared::BenchmarkResponse::ok("Saved")
}

fn save_upload(name: &str, data: &[u8]) {
    let _ = std::fs::write(format!("/var/uploads/{}", name), data);
}
