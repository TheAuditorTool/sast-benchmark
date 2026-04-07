pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content = req.param("content");
    let dest = format!("/uploads/{}", filename);
    let _ = std::fs::write(&dest, content.as_bytes());
    super::shared::BenchmarkResponse::ok("Written")
}
