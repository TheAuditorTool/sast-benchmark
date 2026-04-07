pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content = req.param("content");
    if content.len() > 10_485_760 {
        return super::shared::BenchmarkResponse::bad_request("Too large");
    }
    let _ = std::fs::write(format!("/uploads/{}", filename), content.as_bytes());
    super::shared::BenchmarkResponse::ok("Saved")
}
