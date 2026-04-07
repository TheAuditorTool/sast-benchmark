pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let path = req.param("path");

    match std::fs::metadata(&path) {
        Ok(meta) => super::shared::BenchmarkResponse::ok(&format!("size={}", meta.len())),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
