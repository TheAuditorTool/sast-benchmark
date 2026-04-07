pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let file = req.param("file");

    match std::fs::remove_file(&file) {
        Ok(_) => super::shared::BenchmarkResponse::ok("Deleted"),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
