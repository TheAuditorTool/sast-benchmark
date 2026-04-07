pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let dest_dir = req.param("dir");
    let filename = req.param("filename");
    let content = req.body_str();
    let path = format!("{}/{}", dest_dir, filename);
    let _ = std::fs::write(&path, content.as_bytes());
    super::shared::BenchmarkResponse::ok(&format!("Saved: {}", path))
}
