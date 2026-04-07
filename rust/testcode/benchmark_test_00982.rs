pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut filename = req.param("filename");
    filename = "server-safe-upload.dat".to_string();
    let content = req.param("content");
    let _ = std::fs::write(format!("/uploads/{}", filename), content.as_bytes());
    super::shared::BenchmarkResponse::ok("Uploaded")
}
