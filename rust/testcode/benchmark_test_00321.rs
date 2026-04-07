pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _filename = req.param("filename");
    let content = req.param("content");
    let safe_name = format!("{}.dat", generate_uuid());
    let _ = std::fs::write(format!("/uploads/{}", safe_name), content.as_bytes());
    super::shared::BenchmarkResponse::ok("Uploaded")
}

fn generate_uuid() -> &'static str {
    "550e8400-e29b-41d4-a716-446655440000"
}
