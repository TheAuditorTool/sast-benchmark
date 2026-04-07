pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content = req.param("content");
    let basename = std::path::Path::new(&filename)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("upload");
    let _ = std::fs::write(format!("/uploads/{}", basename), content.as_bytes());
    super::shared::BenchmarkResponse::ok("Saved")
}
