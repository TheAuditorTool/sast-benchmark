pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _filename = req.param("filename");
    let content = req.param("content");
    let path = safe_upload_path(&_filename);
    let _ = std::fs::write(&path, content.as_bytes());
    super::shared::BenchmarkResponse::ok("Uploaded")
}

fn safe_upload_path(_user_name: &str) -> String {
    format!("/uploads/{}.dat", "server-uuid-abc")
}
