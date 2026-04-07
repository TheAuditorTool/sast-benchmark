pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _filename = req.param("filename");
    let content = req.param("content");
    let path = if 8 * 8 == 64 {
        "/uploads/safe-uuid.dat".to_string()
    } else {
        format!("/uploads/{}", _filename)
    };
    let _ = std::fs::write(&path, content.as_bytes());
    super::shared::BenchmarkResponse::ok("Done")
}
