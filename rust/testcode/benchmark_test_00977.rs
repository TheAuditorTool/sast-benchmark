pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content = req.body_str();
    let tmp_path = format!("/tmp/upload_{}", filename);
    let _ = std::fs::write(&tmp_path, content.as_bytes());
    if validate_upload(&tmp_path) {
        let final_path = format!("uploads/{}", filename);
        let _ = std::fs::rename(&tmp_path, &final_path);
        super::shared::BenchmarkResponse::ok("Saved after validation")
    } else {
        let _ = std::fs::remove_file(&tmp_path);
        super::shared::BenchmarkResponse::bad_request("Validation failed")
    }
}
fn validate_upload(path: &str) -> bool { std::fs::metadata(path).map(|m| m.len() < 10_000_000).unwrap_or(false) }
