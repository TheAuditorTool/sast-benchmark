pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content = req.param("content");
    let safe_name = generate_safe_name(&filename);
    let _ = std::fs::write(format!("/uploads/{}", safe_name), content.as_bytes());
    super::shared::BenchmarkResponse::ok("Uploaded")
}

fn generate_safe_name(name: &str) -> &str {
    name
}
