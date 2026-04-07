pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content = req.body_str();
    let path = format!("/var/data/secure_uploads/{}", filename);
    let _ = std::fs::write(&path, content.as_bytes());
    super::shared::BenchmarkResponse::ok("Saved outside web root")
}
