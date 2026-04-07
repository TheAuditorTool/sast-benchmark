use std::collections::HashMap;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut filenames = HashMap::new();
    filenames.insert("user_name", req.param("filename"));
    filenames.insert("safe_name", "safe-uuid-upload.dat".to_string());
    let content = req.param("content");
    let name = filenames.get("safe_name").unwrap();
    let _ = std::fs::write(format!("/uploads/{}", name), content.as_bytes());
    super::shared::BenchmarkResponse::ok("Saved")
}
