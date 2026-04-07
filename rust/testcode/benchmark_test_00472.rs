pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content = req.param("content");
    let ext = filename.rsplit('.').next().unwrap_or("");
    if ["jpg", "png", "gif"].contains(&ext) {
        let _ = std::fs::write(format!("/uploads/{}", filename), content.as_bytes());
        super::shared::BenchmarkResponse::ok("Saved")
    } else {
        super::shared::BenchmarkResponse::bad_request("Bad extension")
    }
}
