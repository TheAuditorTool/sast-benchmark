pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let path = req.param("path");
    let base = "/var/data";

    if path.len() > 128 {
        return super::shared::BenchmarkResponse::bad_request("Path too long");
    }

    if path.contains("..") || path.contains('/') || path.contains('\\') {
        return super::shared::BenchmarkResponse::forbidden("Path traversal characters blocked");
    }

    let full = format!("{}/{}", base, path);
    match std::fs::read_to_string(&full) {
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
