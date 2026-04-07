pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content_type = req.header("content-type");
    let content = req.body_str();
    let ext = filename.rsplit('.').next().unwrap_or("");
    if !triple_check(&content_type, content.as_bytes(), ext) {
        return super::shared::BenchmarkResponse::bad_request("Failed validation");
    }
    let _ = std::fs::write(&format!("uploads/{}", filename), content.as_bytes());
    super::shared::BenchmarkResponse::ok("Saved")
}
fn triple_check(ct: &str, data: &[u8], ext: &str) -> bool {
    let ct_ok = ct == "image/jpeg" || ct == "image/png";
    let magic_ok = data.starts_with(&[0xFF, 0xD8]) || data.starts_with(&[0x89, 0x50]);
    let ext_ok = ext == "jpg" || ext == "jpeg" || ext == "png";
    ct_ok && magic_ok && ext_ok
}
