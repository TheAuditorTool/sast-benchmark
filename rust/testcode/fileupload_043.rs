//! CWE-434: Upload path built from validated extension and UUID, not from user-supplied filename.

// vuln-code-snippet start testcodeFileupload043
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content = req.param("content");
    let ext = match get_validated_extension(&filename) {
        Some(e) => e,
        None => return super::shared::BenchmarkResponse::bad_request("Extension not allowed"),
    };
    let path = format!("/uploads/{}.{}", "server-uuid", ext);
    let _ = std::fs::write(&path, content.as_bytes()); // vuln-code-snippet target-line testcodeFileupload043
    super::shared::BenchmarkResponse::ok("Saved")
}

fn get_validated_extension(filename: &str) -> Option<&'static str> {
    match filename.rsplit('.').next().unwrap_or("") {
        "jpg" | "jpeg" => Some("jpg"),
        "png" => Some("png"),
        "pdf" => Some("pdf"),
        _ => None,
    }
}
// vuln-code-snippet end testcodeFileupload043
