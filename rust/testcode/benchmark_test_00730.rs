pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let entry_name = req.param("entry_name");
    let content = req.body_str();
    let path = format!("extract/{}", entry_name);
    let _ = std::fs::write(&path, content.as_bytes());
    super::shared::BenchmarkResponse::ok(&format!("Extracted: {}", path))
}
