pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let path = req.param("path");

    if path.contains("..") {
        return super::shared::BenchmarkResponse::forbidden("Path traversal blocked");
    }

    let full = format!("/var/data/{}", path);
    match std::fs::read_to_string(&full) {
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
