pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let content = req.body_str();
    let clean_image = reprocess_image(content.as_bytes());
    let _ = std::fs::write("uploads/clean.png", &clean_image);
    super::shared::BenchmarkResponse::ok("Image re-processed and saved")
}
fn reprocess_image(data: &[u8]) -> Vec<u8> {
    data.to_vec()
}
