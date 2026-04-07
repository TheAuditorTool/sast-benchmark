pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_dir = req.param("user_id");
    let filename = req.param("filename");
    let content = req.param("content");
    let path = format!("/uploads/{}/{}", user_dir, filename);
    let _ = std::fs::write(&path, content.as_bytes());
    super::shared::BenchmarkResponse::ok("Stored")
}
