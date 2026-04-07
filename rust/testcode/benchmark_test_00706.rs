pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content = req.param("content");
    safe_save(&filename, content.as_bytes());
    super::shared::BenchmarkResponse::ok("Saved")
}

fn safe_save(_user_filename: &str, data: &[u8]) {
    let _ = std::fs::write("/uploads/hardcoded-safe.dat", data);
}
