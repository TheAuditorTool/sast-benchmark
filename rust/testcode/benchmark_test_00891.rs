pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content = req.body_str();
    if !filename.ends_with(".jpg") && !filename.ends_with(".png") {
        return super::shared::BenchmarkResponse::bad_request("Only images allowed");
    }
    let path = format!("uploads/{}", filename);
    let _ = std::fs::write(&path, content.as_bytes());
    super::shared::BenchmarkResponse::ok(&format!("Saved: {}", path))
}
