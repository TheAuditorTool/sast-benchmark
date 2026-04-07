pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content = req.param("content");
    if filename.ends_with(".jpg") {
        let _ = std::fs::write(format!("/uploads/{}", filename), content.as_bytes());
        super::shared::BenchmarkResponse::ok("Image saved")
    } else {
        super::shared::BenchmarkResponse::bad_request("JPG only")
    }
}
