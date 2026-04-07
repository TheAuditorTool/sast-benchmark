pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let data = req.param("data");
    let path = format!("/uploads/{}", filename);
    let _ = std::fs::write(&path, data.as_bytes());
    super::shared::BenchmarkResponse::ok("Uploaded")
}
