pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content = req.body_str();
    let ext = filename.rsplit('.').next().unwrap_or("");
    if ext == "txt" || ext == "csv" || ext == "log" {
        let path = format!("uploads/{}", filename);
        let _ = std::fs::write(&path, content.as_bytes());
        super::shared::BenchmarkResponse::ok(&format!("Saved: {}", path))
    } else {
        super::shared::BenchmarkResponse::bad_request("Unsupported file type")
    }
}
