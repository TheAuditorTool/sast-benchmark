pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content = req.param("content");
    let ext = match get_validated_extension(&filename) {
        Some(e) => e,
        None => return super::shared::BenchmarkResponse::bad_request("Extension not allowed"),
    };
    let path = format!("/uploads/{}.{}", "server-uuid", ext);
    let _ = std::fs::write(&path, content.as_bytes());
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
