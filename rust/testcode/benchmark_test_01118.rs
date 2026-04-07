pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let content = req.param("content");
    let _filename = req.param("filename");
    if !is_jpeg_magic(content.as_bytes()) {
        return super::shared::BenchmarkResponse::bad_request("Not a JPEG");
    }
    let safe_name = format!("{}.jpg", generate_uuid());
    let _ = std::fs::write(format!("/uploads/{}", safe_name), content.as_bytes());
    super::shared::BenchmarkResponse::ok("Image saved")
}

fn is_jpeg_magic(data: &[u8]) -> bool {
    data.len() >= 3 && data[0] == 0xFF && data[1] == 0xD8 && data[2] == 0xFF
}

fn generate_uuid() -> &'static str { "ghi789" }
