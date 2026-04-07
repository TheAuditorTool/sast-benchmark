pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let dir = req.param("dir");
    let filename = req.param("filename");

    let allowed_dirs = ["/var/data/public", "/var/data/docs"];

    if !allowed_dirs.contains(&dir.as_str()) {
        return super::shared::BenchmarkResponse::forbidden("Directory not allowed");
    }

    if filename.contains("..") || filename.contains('/') || filename.contains('\\') {
        return super::shared::BenchmarkResponse::forbidden("Invalid filename");
    }

    let full = format!("{}/{}", dir, filename);
    match std::fs::read_to_string(&full) {
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
