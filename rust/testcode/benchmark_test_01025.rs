fn is_safe_filename(s: &str) -> bool {
    !s.contains("..") && !s.contains('/') && !s.contains('\\')
        && s.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '-' || c == '_')
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("file");

    if !is_safe_filename(&filename) {
        return super::shared::BenchmarkResponse::forbidden("Invalid filename");
    }

    let path = format!("/uploads/{}", filename);
    match std::fs::read_to_string(&path) {
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
