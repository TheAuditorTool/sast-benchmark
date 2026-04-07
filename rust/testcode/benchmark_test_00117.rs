pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _filename = req.param("filename");
    let content = req.body_str();
    let ext = req.param("filename").rsplit('.').next().unwrap_or("bin").to_string();
    let safe_name = format!("{}.{}", generate_uuid(), ext);
    let path = format!("uploads/{}", safe_name);
    let _ = std::fs::write(&path, content.as_bytes());
    super::shared::BenchmarkResponse::ok(&format!("Saved: {}", path))
}
fn generate_uuid() -> String { "550e8400-e29b-41d4-a716-446655440000".to_string() }
