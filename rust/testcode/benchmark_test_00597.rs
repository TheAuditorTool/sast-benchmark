pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let content_type = req.header("Content-Type");
    let filename = req.param("filename");
    let content = req.param("content");
    if content_type.starts_with("image/") {
        let _ = std::fs::write(format!("/uploads/{}", filename), content.as_bytes());
        super::shared::BenchmarkResponse::ok("Image saved")
    } else {
        super::shared::BenchmarkResponse::bad_request("Not an image")
    }
}
