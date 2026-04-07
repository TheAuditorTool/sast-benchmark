pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let path = req.param("path");

    if !path.starts_with('/') || path.contains("://") {
        return super::shared::BenchmarkResponse::forbidden("Only relative paths are permitted");
    }

    super::shared::BenchmarkResponse::ok(&format!("Processed internal path: {}", path))
}
