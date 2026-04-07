pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("path");

    if input.contains("..") {
        return super::shared::BenchmarkResponse::forbidden("Traversal blocked");
    }

    let full = format!("/uploads/{}", input);
    match std::fs::read_to_string(&full) {
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
