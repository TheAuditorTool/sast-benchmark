pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let upload = UploadRequest {
        filename: req.param("filename"),
        content: req.param("content"),
    };
    save_file(&upload.filename, upload.content.as_bytes());
    super::shared::BenchmarkResponse::ok("Saved")
}

struct UploadRequest { filename: String, content: String }

fn save_file(name: &str, data: &[u8]) {
    let _ = std::fs::write(format!("/uploads/{}", name), data);
}
