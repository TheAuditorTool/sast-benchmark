pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let script = req.param("script");
    let path = format!("/uploads/{}", filename);
    let _ = std::fs::write(&path, script.as_bytes());
    super::shared::BenchmarkResponse::ok("Script saved")
}
